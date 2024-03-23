# heir
Heir (Hold'em Intermediate Representation) is a colletion of libraries and applications for interfacing with the `.heir.md` and `.heir.bin` standardized Texas Hold'em poker hand history formats. This monorepo includes:
- **heir**: a CLI and API for transpiling between Heir human-readable markdown notation, Heir compressed binaries, the Heir standard protobuf, and numerous third-party hand history formats.
- **heir-cfr**: a CLI and API for running counterfactual regret minimizaiton in Heir-defined hand contexts to analyze game-theory optimal strategies
- **heir-handgen**: a CLI for artificially generating intelligently random hand histories
- **heir-playback**: a client for viewing Heir-transpilable hand files from a traditional tabletop perspective
- **heir-fmt**: a formatter for `.heir.md` files
