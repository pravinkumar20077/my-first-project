📝 Project Description
The Charity Donation Tracker is a blockchain solution designed to bring 100% transparency to philanthropy. Traditional charity systems often lack clear visibility into how much is raised and who has contributed. By moving this logic to a Soroban smart contract, every donation becomes a permanent part of the Stellar ledger, ensuring that funds are accounted for and goals are visible to the public.

🚀 What it does
Initializes Beneficiaries: Sets a specific charity wallet as the authorized receiver.

Live Tracking: Aggregates total donations in real-time on-chain.

Donor Ledger: Maintains a detailed history of how much each individual address has contributed.

Public Auditing: Provides read-only functions for anyone to verify the campaign's progress without requiring a centralized API.

✨ Features
Permissionless Transparency: Anyone can query the total funds raised.

Secure Auth: Integration with Soroban’s require_auth ensures that donor identity is cryptographically verified.

Efficient State Management: Uses a mix of Instance and Persistent storage to optimize for ledger costs.

Scalable: Designed to handle thousands of unique donors with minimal latency.

🛠 Tech Stack
Smart Contract: Soroban (Rust-based)

Blockchain: Stellar

Development Kit: Soroban SDK

⚙️ Getting Started
1. Setup Environment
Ensure you have the Soroban CLI and Rust installed:

Bash
rustup target add wasm32-unknown-unknown
cargo install --locked soroban-cli
2. Build the Contract
Bash
soroban contract build
3. Test the Logic
Bash
cargo test
🔗 Deployed Smart Contract Link
Network: Stellar Testnet

Contract ID: [Insert-Your-Generated-Contract-ID]

Explorer: Stellar.Expert

🤝 Contributing
As a B.Tech project, this is open to further enhancements! Feel free to fork the repo and submit pull requests for features like:

Integrating the Stellar Asset Contract (SAC) for real XLM transfers.

Adding "Deadline" logic for time-limited fundraising.

Implementing a frontend using the soroban-client JS library.

Developed by Parvin Aspiring Blockchain Developer | B.Tech Student


1. Contract address:CBHVSRAITFIYLODG4D5INKTDOEMM7HLLGE3KBSODYB3PAFQIPSXTHJ4F
	2. Wallet address:GBGXIGLYX6IPSHIDM522CQJSTTB4PSGX57QYK5HMH3OASTVM72B32J4R
	3. Image:<img width="1919" height="1079" alt="image" src="https://github.com/user-attachments/assets/d210014b-379c-44e1-8aba-23d848b7871a" />

	4. Link:https://stellar.expert/explorer/testnet/contract/CBHVSRAITFIYLODG4D5INKTDOEMM7HLLGE3KBSODYB3PAFQIPSXTHJ4F
