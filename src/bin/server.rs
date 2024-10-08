/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ----------------------------------------------------------------

use tonic::transport::Server;

use hello_grpc::grpc::AuthServiceImpl;
use hello_grpc::protobuf::auth::v1::auth_service_server::AuthServiceServer;

// ----------------------------------------------------------------

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let grpc = "0.0.0.0:9527".parse()?;
    println!("The gRPC Server listening to {}", grpc);

    Server::builder()
        .add_service(AuthServiceServer::new(AuthServiceImpl))
        .serve(grpc)
        .await?;

    Ok(())
}
