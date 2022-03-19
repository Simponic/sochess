# SoChess is Web-Socket Chess

This is a project for HackUSU 2022 using MongoDB, Express, and Rust for a CLI client that we were hoping to use a SocketIO binding with.

Is @CodeTriangle proud of the work done for the Rust chess CLI? Yes. And it looks really good I must say.

Am I proud of the work I did on the WebSockets and Auth API? Meh. I didn't sleep at all and the code is full of smells. My expectations for the work I could do with no sleep with a completely foreign tech stack was way too high.

Did the client and server end up ever even being integrated together? Nope. And it would take a lot longer to do so.

Missing features:
* No documentation
* No win/loss/stalemate state
* No castling
* No promoting of pawns
* No UI client to interact with the server.
* Not much information sent to the the client about game.
* No timers

Semi-working features:
* Username/password registration and auth
* En-passant
* Joining another game and playing in real time
* Others that I don't want to list cuz I'm the most tired I've been in my life (26 hours up and counting!).
