# 📦 Relay File Transfer

A simple file transfer utility using [Iroh](https://github.com/n0-computer/iroh), built in Rust. This tool allows you to send and receive files using a decentralized approach. 🚀

## ✨ Features
- 📤 **Send files** over Iroh
- 📥 **Receive files** with a secure ticket system
- ⏳ **Download timeout** for better reliability
- 🔗 **Uses Iroh’s blob storage** for efficient transfers

## 🛠️ Installation
Ensure you have **Rust** installed. If not, install it using [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone this repository and navigate into the project folder:

```sh
git clone git@github.com:shuklarituparn/relay_file_transfer.git
cd relay_file_transfer
```

## 🚀 Usage

### 📤 Sending a File
To send a file, run:

```sh
cargo run -- send path/to/file
```

This will generate a **ticket**. Share this ticket with the receiver, and the file name.

### 📥 Receiving a File
To receive a file, use the ticket shared by the sender, and the file name (Which will be the filename when saving the file):

```sh
cargo run -- receive [TICKET] path/to/destination
```

## ⚙️ How It Works
1. **Send a file**: The file is analyzed and stored in the Iroh blob storage (in memory).
2. **Generate a ticket**: The ticket contains information to retrieve the file.
3. **Receive the file**: The receiver downloads the file using the ticket.
4. **Save the file**: The received data is written to the specified destination.

## ⏳ Timeout Handling
To prevent infinite waiting, the download process includes a **30-second timeout**. If the file cannot be retrieved within this time, the process will abort.

## 🛑 Shutting Down
After transferring the files, the Iroh node will shut down gracefully.

## 🔧 Dependencies
- [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) for error handling
- [`iroh`](https://github.com/n0-computer/iroh) for decentralized file storage
- [`tokio`](https://tokio.rs/) for async runtime

## 🤝 Contributing
Feel free to fork, submit PRs, or open issues! 🚀

## 📜 License
[License](./LICENSE)

