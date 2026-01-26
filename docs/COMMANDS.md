# FrogPump CLI Command Reference

## Global Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Enable debug-level logging output |
| `--network <NETWORK>` | `-n` | Target network: mainnet, devnet, localnet |
| `--config <PATH>` | `-c` | Path to custom config file |
| `--help` | `-h` | Print help information |
| `--version` | `-V` | Print version information |

---

## Token Management

### `frogpump launch`

Launch a new token on Solana via pump.fun.

**Usage:**
```
frogpump launch --name <NAME> --symbol <SYMBOL> [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `--name <NAME>` | Yes | Token display name (3-32 characters) |
| `--symbol <SYMBOL>` | Yes | Token ticker symbol (2-10 characters, uppercase) |
| `--description <DESC>` | No | Token description text |
| `--image <URL>` | No | URL to token image |
| `--gasless` | No | Use gasless launch (default: true) |

**Examples:**
```bash
frogpump launch --name "FrogCoin" --symbol "FROG"
frogpump launch --name "PepeAI" --symbol "PEPAI" --description "AI-powered meme token"
frogpump launch --name "SolFrog" --symbol "SFROG" --no-gasless
```

### `frogpump tokens`

List all tokens launched by the current agent.

**Usage:**
```
frogpump tokens [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `--agent-id <ID>` | No | Filter by agent ID (defaults to configured agent) |
| `--limit <N>` | No | Maximum number of tokens to display (default: 20) |
| `--format <FMT>` | No | Output format: table, json, csv |

**Examples:**
```bash
frogpump tokens
frogpump tokens --limit 5 --format json
frogpump tokens --agent-id agent_abc123
```

### `frogpump token <MINT_ADDRESS>`

Show detailed information about a specific token.

**Usage:**
```
frogpump token <MINT_ADDRESS>
```

**Examples:**
```bash
frogpump token 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
```

---

## Earnings

### `frogpump earnings`

View earnings from launched tokens.

**Usage:**
```
frogpump earnings [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `--token-id <ID>` | No | Filter earnings by token ID |
| `--unclaimed` | No | Show only unclaimed earnings |
| `--format <FMT>` | No | Output format: table, json |

**Examples:**
```bash
frogpump earnings
frogpump earnings --unclaimed
frogpump earnings --token-id tok_001 --format json
```

### `frogpump claim`

Claim pending earnings to your wallet.

**Usage:**
```
frogpump claim [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `--earning-id <ID>` | No | Claim a specific earning (claims all if omitted) |
| `--dry-run` | No | Simulate the claim without executing |

**Examples:**
```bash
frogpump claim
frogpump claim --earning-id earn_001
frogpump claim --dry-run
```

---

## Configuration

### `frogpump config set`

Set a configuration value.

**Usage:**
```
frogpump config set <KEY> <VALUE>
```

**Examples:**
```bash
frogpump config set agent_id agent_abc123
frogpump config set network devnet
frogpump config set rpc_url https://my-rpc.example.com
frogpump config set verbose true
```

### `frogpump config get`

Get a configuration value.

**Usage:**
```
frogpump config get <KEY>
```

**Examples:**
```bash
frogpump config get network
frogpump config get rpc_url
```

### `frogpump config show`

Display all current configuration values.

**Usage:**
```
frogpump config show
```

### `frogpump config reset`

Reset configuration to defaults.

**Usage:**
```
frogpump config reset [--confirm]
```

---

## Wallet

### `frogpump wallet`

Display wallet information and balance.

**Usage:**
```
frogpump wallet [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `--address <ADDR>` | No | Check a specific wallet address |

**Examples:**
```bash
frogpump wallet
frogpump wallet --address 9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM
```

---

## Status

### `frogpump status`

Check API and network connectivity status.

**Usage:**
```
frogpump status
```

**Output includes:**
- API endpoint reachability
- Current network and RPC endpoint
- Configured agent ID
- Wallet connection status
