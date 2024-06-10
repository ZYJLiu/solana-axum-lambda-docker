Working locally to build 0.30.0 with new IDL, and declare_program()

request with idl for declare_program()

```json
{
  "files": [
    [
      "/src/lib.rs",
      "use anchor_lang::prelude::*;\n\ndeclare_id!(\"Bi5N7SUQhpGknVcqPTzdFFVueQoxoUu8YTLz75J6fT8A\");\n\n// Automatically generate module using program IDL found in ./idls\ndeclare_program!(lever);\n\nuse lever::accounts::PowerStatus;\nuse lever::cpi::accounts::SwitchPower;\nuse lever::cpi::switch_power;\nuse lever::program::Lever;\n\n#[program]\npub mod hand {\n    use super::*;\n\n    pub fn pull_lever(ctx: Context<PullLever>, name: String) -> Result<()> {\n        let cpi_ctx = CpiContext::new(\n            ctx.accounts.lever_program.to_account_info(),\n            SwitchPower {\n                power: ctx.accounts.power.to_account_info(),\n            },\n        );\n        switch_power(cpi_ctx, name)?;\n        Ok(())\n    }\n}\n\n#[derive(Accounts)]\npub struct PullLever<'info> {\n    #[account(mut)]\n    pub power: Account<'info, PowerStatus>,\n    pub lever_program: Program<'info, Lever>,\n}\n"
    ],
    [
      "/idls/lever.json",
      "{\n  \"address\": \"E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN\",\n  \"metadata\": {\n    \"name\": \"lever\",\n    \"version\": \"0.1.0\",\n    \"spec\": \"0.1.0\",\n    \"description\": \"Created with Anchor\"\n  },\n  \"instructions\": [\n    {\n      \"name\": \"initialize\",\n      \"discriminator\": [175, 175, 109, 31, 13, 152, 155, 237],\n      \"accounts\": [\n        {\n          \"name\": \"power\",\n          \"writable\": true,\n          \"signer\": true\n        },\n        {\n          \"name\": \"user\",\n          \"writable\": true,\n          \"signer\": true\n        },\n        {\n          \"name\": \"system_program\",\n          \"address\": \"11111111111111111111111111111111\"\n        }\n      ],\n      \"args\": []\n    },\n    {\n      \"name\": \"switch_power\",\n      \"discriminator\": [226, 238, 56, 172, 191, 45, 122, 87],\n      \"accounts\": [\n        {\n          \"name\": \"power\",\n          \"writable\": true\n        }\n      ],\n      \"args\": [\n        {\n          \"name\": \"name\",\n          \"type\": \"string\"\n        }\n      ]\n    }\n  ],\n  \"accounts\": [\n    {\n      \"name\": \"PowerStatus\",\n      \"discriminator\": [145, 147, 198, 35, 253, 101, 231, 26]\n    }\n  ],\n  \"types\": [\n    {\n      \"name\": \"PowerStatus\",\n      \"type\": {\n        \"kind\": \"struct\",\n        \"fields\": [\n          {\n            \"name\": \"is_on\",\n            \"type\": \"bool\"\n          }\n        ]\n      }\n    }\n  ]\n}\n"
    ]
  ],
  "uuid": null,
  "flags": {
    "seeds_feature": false,
    "no_docs": true,
    "safety_checks": false
  }
}
```

response:

```json
{
  "stderr": "    Finished release [optimized] target(s) in 0.09s\n",
  "uuid": "b9dd0ed1-c27d-465c-b588-44d865da1fcb",
  "idl": {
    "address": "Bi5N7SUQhpGknVcqPTzdFFVueQoxoUu8YTLz75J6fT8A",
    "metadata": {
      "name": "hand",
      "version": "0.1.0",
      "spec": "0.1.0"
    },
    "instructions": [
      {
        "name": "pull_lever",
        "discriminator": [137, 127, 205, 31, 6, 132, 54, 97],
        "accounts": [
          {
            "name": "power",
            "writable": true
          },
          {
            "name": "lever_program"
          }
        ],
        "args": [
          {
            "name": "name",
            "type": "string"
          }
        ]
      }
    ],
    "accounts": [
      {
        "name": "PowerStatus",
        "discriminator": [145, 147, 198, 35, 253, 101, 231, 26]
      }
    ],
    "types": [
      {
        "name": "PowerStatus",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "is_on",
              "type": "bool"
            }
          ]
        }
      }
    ]
  }
}
```

request with regular program

```json
{
  "files": [
    [
      "/src/lib.rs",
      "use anchor_lang::prelude::*;\n\n// This is your program's public key and it will update\n// automatically when you build the project.\ndeclare_id!(\"3qn2eBQ7UKJirQzq2Km88ek1VDfzcTFaUHuCuuJhJFMV\");\n\n#[program]\nmod hello_anchor {\n    use super::*;\n    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {\n        ctx.accounts.new_account.data = data;\n        msg!(\"Changed data to: {}!\", data); // Message will show up in the tx logs\n        Ok(())\n    }\n}\n\n#[derive(Accounts)]\npub struct Initialize<'info> {\n    // We must specify the space in order to initialize an account.\n    // First 8 bytes are default account discriminator,\n    // next 8 bytes come from NewAccount.data being type u64.\n    // (u64 = 64 bits unsigned integer = 8 bytes)\n    #[account(init, payer = signer, space = 8 + 8)]\n    pub new_account: Account<'info, NewAccount>,\n    #[account(mut)]\n    pub signer: Signer<'info>,\n    pub system_program: Program<'info, System>,\n}\n\n#[account]\npub struct NewAccount {\n    data: u64\n}"
    ]
  ],
  "uuid": null,
  "flags": {
    "seeds_feature": false,
    "no_docs": true,
    "safety_checks": false
  }
}
```

response:

```json
{
  "stderr": "   Compiling solpg v0.1.0 (/programs)\n    Finished release [optimized] target(s) in 1.39s\n",
  "uuid": "d3200dc3-c6b9-4071-a2ef-4c27e6202659",
  "idl": {
    "address": "3qn2eBQ7UKJirQzq2Km88ek1VDfzcTFaUHuCuuJhJFMV",
    "metadata": {
      "name": "hello_anchor",
      "version": "0.1.0",
      "spec": "0.1.0"
    },
    "instructions": [
      {
        "name": "initialize",
        "discriminator": [175, 175, 109, 31, 13, 152, 155, 237],
        "accounts": [
          {
            "name": "new_account",
            "writable": true,
            "signer": true
          },
          {
            "name": "signer",
            "writable": true,
            "signer": true
          },
          {
            "name": "system_program"
          }
        ],
        "args": [
          {
            "name": "data",
            "type": "u64"
          }
        ]
      }
    ],
    "accounts": [
      {
        "name": "NewAccount",
        "discriminator": [176, 95, 4, 118, 91, 177, 125, 232]
      }
    ],
    "types": [
      {
        "name": "NewAccount",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "data",
              "type": "u64"
            }
          ]
        }
      }
    ]
  }
}
```

Worked locally but not deployed, AWS Lambda readonly file system
Read-only file system (os error 30)

Adding /tmp/program doesn't work.
Seems /tmp from dockerfile isn't recognized on lambda

```
2024-06-10T05:52:08.770Z	START RequestId: 3b5fd431-4159-4c63-b600-cb8a00ec50e3 Version: $LATEST
2024-06-10T05:52:08.890Z	thread 'tokio-runtime-worker' panicked at src/program.rs:72:60:
2024-06-10T05:52:08.890Z	Could not read manifest: Os { code: 2, kind: NotFound, message: "No such file or directory" }
2024-06-10T05:52:08.890Z	note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2024-06-10T05:52:09.203Z	thread 'main' panicked at src/routes/build.rs:74:6:
2024-06-10T05:52:09.203Z	`spawn_blocking` failure: JoinError::Panic(Id(5), ...)
2024-06-10T05:52:09.295Z	helloERROR Lambda runtime invoke{requestId="3b5fd431-4159-4c63-b600-cb8a00ec50e3" xrayTraceId="Root=1-66669488-0d5ea8712bcf0f5a5d5417cc;Parent=037132a81d1cae9d;Sampled=0;Lineage=c68c977e:0"}: user handler panicked error=Any { .. }
2024-06-10T05:52:09.297Z	END RequestId: 3b5fd431-4159-4c63-b600-cb8a00ec50e3
```

workaround by copying over files into /tmp,
turns out more memory on lambda gets more cpu, builds in 1-3 sec with max memory on lambda

```
    "stderr": "   Compiling solpg v0.1.0 (/tmp/programs)\n    Finished release [optimized] target(s) in 1m 15s\n",
```

http://127.0.0.1:8080/build

`/build` Post Request Body Example
Internal Server Error

```json
{
  "files": [
    [
      "/src/lib.rs",
      "use anchor_lang::prelude::*;\n\n// This is your program's public key and it will update\n// automatically when you build the project.\ndeclare_id!(\"3qn2eBQ7UKJirQzq2Km88ek1VDfzcTFaUHuCuuJhJFMV\");\n\n#[program]\nmod hello_anchor {\n    use super::*;\n    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {\n        ctx.accounts.new_account.data = data;\n        msg!(\"Changed data to: {}!\", data); // Message will show up in the tx logs\n        Ok(())\n    }\n}\n\n#[derive(Accounts)]\npub struct Initialize<'info> {\n    // We must specify the space in order to initialize an account.\n    // First 8 bytes are default account discriminator,\n    // next 8 bytes come from NewAccount.data being type u64.\n    // (u64 = 64 bits unsigned integer = 8 bytes)\n    #[account(init, payer = signer, space = 8 + 8)]\n    pub new_account: Account<'info, NewAccount>,\n    #[account(mut)]\n    pub signer: Signer<'info>,\n    pub system_program: Program<'info, System>,\n}\n\n#[account]\npub struct NewAccount {\n    data: u64\n}"
    ]
  ],
  "uuid": null,
  "flags": {
    "seeds_feature": false,
    "no_docs": true,
    "safety_checks": false
  }
}
```

0.29.0 Response:

```json
{
  "stderr": "   Compiling solpg v0.1.0 (/home/appuser/programs)\n    Finished release [optimized] target(s) in 4.82s\n",
  "uuid": "c2695e5b-87b9-4b07-97f0-cf67029571b2",
  "idl": {
    "version": "0.1.0",
    "name": "hello_anchor",
    "instructions": [
      {
        "name": "initialize",
        "accounts": [
          {
            "name": "newAccount",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "signer",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "data",
            "type": "u64"
          }
        ]
      }
    ],
    "accounts": [
      {
        "name": "NewAccount",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "data",
              "type": "u64"
            }
          ]
        }
      }
    ]
  }
}
```
