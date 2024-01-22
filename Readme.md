# GitHub-Meili-Search
GitHub-Meili-Search is an adapter written in Rust, designed to seamlessly integrate GitHub issues into a MeiliSearch repository, providing a significant boost in search functionality. This adapter harnesses the vector search feature of MeiliSearch to enhance search relevancy and accuracy for GitHub issues.

## Features
- **GitHub Integration**: Connect effortlessly with GitHub repositories and retrieve issues for indexing into MeiliSearch
- **MeiliSearch Vector Search**: Harness the power of MeiliSearch's vector search capability to elevate search quality
- **Customizable Indexing**: Configure indexing options to tailor the integration to your specific requirements
- **Easy Setup**: Straightforward setup process with clear instructions to get you started quickly

## Getting Started
### Prerequisites
Before using GitHub-Meili-Search, make sure you have the following prerequisites installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [MeiliSearch](https://www.meilisearch.com/docs/learn/getting_started/installation), e.g. via running `docker run -it --rm -p 7700:7700 getmeili/meilisearch:latest`

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/CommanderStorm/github-meili-search-rust.git
   cd github-meili-search-rust
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Configure your GitHub and MeiliSearch credentials in the `.env` file:
   ```dotenv
   GITHUB_TOKEN=your_github_token
   MEILISEARCH_URL=http://localhost:7700
   MEILISEARCH_API_KEY=your_meilisearch_api_key
   ```
4. Runing the binary:
   ```sh
   ./target/release/github-meili-search-rust
   ```
   This will start the adapter and initiate the process of fetching GitHub issues for indexing into MeiliSearch.

## Configuration
Customize the indexing process by modifying the configuration options in the config.toml file. Adjust settings such as the GitHub repository, MeiliSearch index name, and indexing frequency to suit your requirements.

## Contributing
Contributions are highly encouraged! If you have ideas for improvements or find any issues, please open an issue or submit a pull request.

## License
This project is licensed under the MIT License. Feel free to use and modify the code according to your needs.

## Acknowledgments
The GitHub-Meili-Search project is inspired by the need for a more efficient and accurate search solution for GitHub issues.

Thank you for choosing GitHub-Meili-Search in Rust! If you have any questions or feedback, please don't hesitate to reach out.