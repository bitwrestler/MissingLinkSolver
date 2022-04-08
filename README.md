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


<H3>Output:</H3>
The executable will give stpe-by step instructions on the solution in the form of:<br>
1. Iteration #
2. Instructional wording
3. Resulting columns and rows diagram

<code>
Iteration: 0
Starting Position
Tw  Ty  Mg  Tg
Mw  My  Tr  Mg
Bw  My  Mr  Mr
__  By  Bg  Br

Iteration: 1
Move square Down
__  Ty  Mg  Tg
Tw  My  Tr  Mg
Mw  My  Mr  Mr
Bw  By  Bg  Br

Iteration: 2
Rotate Top to the Right
Tg  __  Ty  Mg
Tw  My  Tr  Mg
Mw  My  Mr  Mr
Bw  By  Bg  Br
</code>

  .....

When the wording says "Move square Up or Down", that could mean moving in mulitple steps.