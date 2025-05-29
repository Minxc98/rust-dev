//引用 proto对象
use crate::protos::voting::voting_client::VotingClient;
use crate::protos::voting::{VotingRequest, GetVotesRequest};

pub async fn _example() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let mut client = VotingClient::connect("http://[::1]:50051").await?;

    // 发送投票
    let request = tonic::Request::new(VotingRequest {
        url: "https://example.com".to_string(),
        vote: 0, // UP vote
    });

    let response = client.vote(request).await?;
    println!("Vote Response: {:?}", response);

    // 获取投票结果
    let request = tonic::Request::new(GetVotesRequest {
        url: "https://example.com".to_string(),
    });

    let response = client.get_votes(request).await?;
    println!("Get Votes Response: {:?}", response);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protos::voting::voting_server::{Voting, VotingServer};
    use crate::protos::voting::{VotingResponse, GetVotesResponse};
    use std::collections::HashMap;
    use std::sync::Mutex;
    use tonic::{transport::Server, Request, Response, Status};

    #[derive(Debug, Default)]
    struct MockVotingService {
        votes: Mutex<HashMap<String, (i32, i32)>>,
    }

    #[tonic::async_trait]
    impl Voting for MockVotingService {
        async fn vote(
            &self,
            request: Request<VotingRequest>,
        ) -> Result<Response<VotingResponse>, Status> {
            let req = request.into_inner();
            let mut votes = self.votes.lock().unwrap();
            let (up_votes, down_votes) = votes.entry(req.url.clone()).or_insert((0, 0));
            
            match req.vote {
                0 => *up_votes += 1,
                1 => *down_votes += 1,
                _ => return Err(Status::invalid_argument("Invalid vote type")),
            }

            Ok(Response::new(VotingResponse {
                confirmation: format!("Vote recorded for {}", req.url),
            }))
        }

        async fn get_votes(
            &self,
            request: Request<GetVotesRequest>,
        ) -> Result<Response<GetVotesResponse>, Status> {
            let req = request.into_inner();
            let votes = self.votes.lock().unwrap();
            let (up_votes, down_votes) = votes.get(&req.url).unwrap_or(&(0, 0));

            Ok(Response::new(GetVotesResponse {
                up_votes: *up_votes,
                down_votes: *down_votes,
            }))
        }
    }

    #[tokio::test]
    async fn test_voting_service() -> Result<(), Box<dyn std::error::Error>> {
        // 启动测试服务器
        let addr = "[::1]:50051".parse()?;
        let voting_service = MockVotingService::default();
        
        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(VotingServer::new(voting_service))
                .serve(addr)
                .await
                .unwrap();
        });

        // 等待服务器启动
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // 运行客户端测试
        let mut client = VotingClient::connect("http://[::1]:50051").await?;

        // 测试投票
        let request = tonic::Request::new(VotingRequest {
            url: "https://test.com".to_string(),
            vote: 0,
        });
        let response = client.vote(request).await?;
        assert!(response.into_inner().confirmation.contains("test.com"));

        // 测试获取投票
        let request = tonic::Request::new(GetVotesRequest {
            url: "https://test.com".to_string(),
        });
        let response = client.get_votes(request).await?;
        let votes = response.into_inner();
        assert_eq!(votes.up_votes, 1);
        assert_eq!(votes.down_votes, 0);

        // 清理
        server.abort();
        Ok(())
    }
}
