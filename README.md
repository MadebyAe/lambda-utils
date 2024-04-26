# Lambda Utils

![lambda-utils](https://github.com/MadebyAe/lambda-utils/assets/1707438/399667dd-0b27-4b7b-bb24-46a6ba314c63)

Lambda Utils is a collection of utility modules designed to facilitate common tasks when developing AWS Lambda functions using Rust. These utilities are intended to simplify the development process and promote code reuse across different Lambda projects.

## Included Utils

### headers.rs
- **Description:** A utility module for parsing HTTP headers.
- **Usage:** Helps in extracting and processing headers from incoming HTTP requests.

### mongodb.rs
- **Description:** A client helper module for MongoDB operations.
- **Usage:** Facilitates interactions with MongoDB databases, providing an abstraction layer for common operations.

### network.rs
- **Description:** A utility module for obtaining network-related information.
- **Usage:** Allows fetching the IP address of the Lambda function execution environment.

### sqs.rs
- **Description:** An AWS SQS (Simple Queue Service) helper module.
- **Usage:** Simplifies interaction with SQS queues, including sending and receiving messages.

## Getting Started
To start using Lambda Utils in your AWS Rust Lambda projects, follow these steps:

1. Add Lambda Utils as a dependency in your  project:

    ```sh
    cargo add lambda-utils
    ```

2. Import the desired utility modules into your Rust Lambda project:

    ```rust
    // Example: Importing header.rs
    use lambda_utils::headers::{get_header_cookies, get_header_user_agent};
    ```

3. Begin using the utilities in your Lambda function code:

    ```rust
    // Example: Getting the user agent from headers
    let user_agent = get_header_user_agent(request);
    ```

## Contributing
Contributions to Lambda Utils are welcome! If you have suggestions for improvements or additional utility modules, please feel free to open an issue or submit a pull request on the [GitHub repository](https://github.com/MadeByAe/lambda-utils).

## License
This project is licensed under the [MIT License](LICENSE.md). Feel free to use, modify, and distribute this code according to the terms of the license.

## About
Lambda Utils is maintained by (Ae) Angel Estrada. For questions or support, please contact [angel-estrada.com].

---

Made with ❤️ in San Francisco
