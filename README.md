# lesbin

A privacy-first end-to-end encrypted paste service for sharing your dreams, fanfiction, logs and code with only the people you wish to.

## About 

### Features

- **Full end-to-end encryption**, pastes can only be read by the intended recipients and no one else, not even the server.
- **Privacy focused**, no uncessary data is collected about you or your pastes. No accounts, no signup, no identifiable info.
- **Simple interface** with support for syntax highlighting (powered by Highlight.js), raw view, and line wrapping.
- **Paste Expire**, set an expiration time, and your paste will automatically be deleted when you’re done with it.
- **Moderation tools**  users can report pastes they think break the rules. Admins can review these reports through an easy-to-use admin panel.

### How Data is Stored & Encrypted

Everything that doesn't need to be known by the server is encrypted, because if the server doesn't need it, it shouldn't have access to it.

#### End-to-End Encryption

The following data is encrypted, ensuring that only those with the decryption key can access it:

- Paste titles.
- Paste contents
- Paste syntax type.

#### Unencrypted Data

The following data is stored unencrypted for operational purposes:

- Paste identifier.
- Paste expiry time.
- Paste creation time.

#### Hashed Data

The following data is hashed before being stored on the server:

- Paste deletion keys.

### Paste Reporting

Lesbin allows users to report pastes if they believe the content should be reviewed by an administrator. When a report is submitted, the paste's decryption key is temporarily sent from the reporter to the server, allowing administrators to view the content and make a judgment.

- If the report is **dismissed**, the paste's decryption key is removed from the database entirely.
- If the report is **accepted**, the paste and all associated data will be deleted entirely.

## Setup

Self-hosting Lesbin is easy if you're familiar with OCI containers. However, pre-compiled builds are not currently available, so you'll need to do the following:

- Clone the repository locally.
- Use the provided [compose file](compose.yml) and make modifications where needed for your setup
- Configure environment variables using [the frontend's .env.example file](./frontend/.env.example) and [the api's .env.example file](./api/.env.example) as references..

For simple setups it's usually enough to only expose the frontend through a reverse proxy and use your internal server network to access the backend from the frontend.