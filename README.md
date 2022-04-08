<H1>Solver for Rubick's Missing Link Puzzle Game</H1>

![Missing Link Puzzle](/pix.png "Missing Link Puzzle")

<p>This repository is a Rust port of Javascript code from https://www.jaapsch.net/puzzles/missing.htm. I did not originate this algorithm, but simply ported it to Rust and modified it somewhat for command line execution and display.</p>

<code>Usage: missing_link_solver_cmd [starting pattern]</code>

<H3>Format of starting pattern:</H3>
> <code>--col 1- --col 2- --col 3- --col 4-<br>
> tile x 4;tile x 4;tile x 4;tile x 4<br></code>
> example: TwMwBw__;TyMyMyBy;MgTrMrBg;TgMgMrBr<br>
> ; = just a nop charcter to help separte the pattern visually


<H3>Tile specification:</H3>
> A tile is a 2 character specifcation of position and color.<br>
> __ = blank tile position<br>
> <b>Position:</b><br>
> T = Top tile<br>
> M = Middle tile<br>
> B = Bottom tile<br>
> <b>Color:</b><br>
> r = Red<br>
> g = Green<br>
> y = Yellow<br>
> w = White<br>
> <br>
> example: Tr = Top tile that is red





