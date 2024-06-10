Working locally to build 0.30.0 with new IDL

```
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
                "discriminator": [
                    175,
                    175,
                    109,
                    31,
                    13,
                    152,
                    155,
                    237
                ],
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
                "discriminator": [
                    176,
                    95,
                    4,
                    118,
                    91,
                    177,
                    125,
                    232
                ]
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

workaround by copying over files into /tmp, 1m 15 on lambda

```
    "stderr": "   Compiling solpg v0.1.0 (/tmp/programs)\n    Finished release [optimized] target(s) in 1m 15s\n",
```

http://127.0.0.1:8080/build

`/build` Post Request Body Example
Internal Server Error

```
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

Response:

```
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
