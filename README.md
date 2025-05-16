# Provable email game engine
This project is a library to create **provable [*play-by-email*](https://en.wikipedia.org/wiki/Play-by-mail_game) games**, leveraging zero-knowledge proofs. 

We provide an example project: a provable chess game played via email.

**This project is work in progress and might not work as expected. I still need to polish the code.**

## Key features
- **Provable gameplay**: Ensures players are honest by using zero-knowledge proofs.
- **Trustworthy**: Game servers cannot cheat, and players can play against dishonest opponents with confidence.
- **Onchain settlement**: Settle your game on the Hylé chain at low cost and without latency.

Learn more:
- [Generic presentation on the Hyli podcast](https://www.youtube.com/watch?v=QZjBa_WTvc)
- Technical presentation at Real World Cryptography Paris //TODO

## How it works
![Architecture](./architecture.png)

### Step-by-step
1. Challenge a friend to play chess by email.
1. Send each other emails with your chess moves, cc’ing the zkChess referee server on your emails.
1. Using zkEmail and RISC Zero, the referee server generates two proofs:
    * [zkEmail](https://prove.email/) generates proof of the state transition sent by email.
    * The [shakmaty library](https://github.com/niklasf/shakmaty) + [RISC Zero](https://www.risc0.com/) allow for easy proof generation that the final email includes a mate.
4. [Hyli](https://hyli.org) verifies and settles both proofs in a single transaction without requiring the proofs to reference each other.

### Components
- 2 players sending each other emails
- A chess engine processing the emails to find the winning condition
- A referee server receiving emails and generating the final zk proofs
- Hyli to verify and settle the final proofs

## Future improvements
### Planned
- Clean up code

### Ideas
- Add privacy (commit-reveal schemes)
- Add more [play-by-email games](https://en.wikipedia.org/wiki/List_of_play-by-mail_games)
- Onchain side bets
- Provable onchain ELO
- Censorship-resistance of the email server

## How to install and run
> TODO.


## Sponsor

<p align="left">
  <a href="https://hyli.org" target="_blank"> <img src="https://github.com/hyli-org/hyli-assets/blob/main/Logos/Logo/HYLI_WORDMARK_ORANGE.png?raw=true" width="15%", height="15%"/></a>
</p>

*This project is supported by [Hyli](https://hyli.org), the new proof-powered L1 to build the next generation of apps, as part of the [Hyli grant program](https://github.com/hyli-org/hyli/blob/main/GRANTS.md).*
