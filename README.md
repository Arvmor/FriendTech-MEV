# Friend Tech MEV

Track and Simulate pending transactions to detect and snipe freshly joined users on the [Friend Tech](https://friend.tech) Platform. Using Third-Party APIs such as Twitter, we can analyze the new users and cherry-pick the Influencer/well-known accounts. 

### Features

- **Simulations** to identify pending transactions related to the Friend Tech ecosystem.
- **Real-time Pending Transaction** for EVM chains to take action before they land on the chain.
- **Customizable strategies** tailored to user-defined parameters.
- **Seamless integration** with Friend Tech’s APIs and blockchain data sources.

## Getting Started

Follow these steps to set up **friend-tech-mev** on your machine.

### Prerequisites

Ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- Ethereum Node
- **friend-tech-mev** works best on Unix-based systems (Linux/Mac).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Arvmor/friend-tech-mev.git
   cd friend-tech-mev
   ```

2. Install Rust dependencies:
   ```bash
   cargo build --release
   ```

3. Set up environment variables

## Usage

Run the bot in release mode to start optimizing Friend Tech transactions:

```bash
cargo run --release
```

## Contributing

We welcome contributions to improve **friend-tech-mev**! To contribute:

1. Fork the repo and create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes, then commit and push:
   ```bash
   git commit -m "Add new feature"
   git push origin feature/your-feature-name
   ```

3. Submit a pull request for review.

### License

Distributed under the MIT License. See `LICENSE` for more information.
