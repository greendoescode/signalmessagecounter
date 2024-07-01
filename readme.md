# Signal Message Counter

Signal Message Counter is a tool for analyzing and visualizing messages from the Signal desktop application stored in an SQLite database.

## Getting Started

### 1. Obtain the SQLite Database

Depending on your operating system, locate the Signal SQLite database:
- **Linux**: `~/.config/Signal/sql/db.sqlite`
- **Mac**: `~/Library/Application Support/Signal/sql/db.sqlite`
- **Windows**: `C:\Users\<YourName>\AppData\Roaming\Signal\sql\db.sqlite` (exact path may vary)

### 2. Decrypt and Convert SQLite Database to CSV

The Signal SQLite database is encrypted using SQLCipher. You need to decrypt it and convert it to CSV format using the following steps:

#### Prerequisites

Ensure you have the following installed:
- **sqlcipher**: A recent version (e.g., 3.31.0 or later) to decrypt the encrypted SQLite database.

#### Decrypt Script

Download the decryption script [sqlite-to-csv.sh](./scripts/sqlite-to-csv.sh) and follow these steps:

1. Save the script in a directory of your choice.
2. Make it executable: `chmod +x sqlite-to-csv.sh`.
3. Execute the script: `bash sqlite-to-csv.sh`.

This script decrypts the Signal SQLite database using the encryption key obtained from `config.json`, extracts message data in JSON format, and writes it to `backup-desktop.csv`. 

To use this script on windows, either use WSL or transfer the files to a linux machine.

### Analyzing Messages

Once you have `backup-desktop.csv`, you can use this tool to parse and analyze the CSV file to extract insights such as message counts, timestamps, and message types (incoming/outgoing). You'll need to get your conversation id for this, which you can find by getting most recent message from your selected chat, doing CTRL-F in the CSV file and on the same line, finding the conversation id. It'll look something like `d37673ee-9d18-4294-be46-30e755a1406`

`Usage: signalmessagecounter <filepath> <conversationId>`

### License

This project is licensed under the BSD 2-Clause License - see the [LICENSE](./LICENSE) file for details.
