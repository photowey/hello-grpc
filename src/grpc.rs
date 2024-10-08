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

use crate::protobuf::auth::v1::{LoginRequest, LoginResponse, LogoutRequest, LogoutResponse, TokenType};
use crate::protobuf::auth::v1::auth_service_server::AuthService;

// ----------------------------------------------------------------

pub struct AuthServiceImpl;

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(
        &self,
        request: tonic::Request<LoginRequest>,
    ) -> Result<tonic::Response<LoginResponse>, tonic::Status> {
        let request = request.into_inner();
        println!("Login request is {:?}", request);

        let response = LoginResponse {
            access_token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c".to_string(),
            token_type: TokenType::Bearer as i32,
        };

        Ok(tonic::Response::new(response))
    }

    async fn logout(
        &self,
        request: tonic::Request<LogoutRequest>,
    ) -> Result<tonic::Response<LogoutResponse>, tonic::Status> {
        let request = request.into_inner();
        println!("Logout request is {:?}", request);

        let response = LogoutResponse {
            code: 200,
            ..Default::default()
        };

        Ok(tonic::Response::new(response))
    }
}
