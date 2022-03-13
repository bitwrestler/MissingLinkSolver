// Javascript written by Jaap Scherphuis. (jaapsch a t yahoo d o t com)

var imagedir="images/missing/";
function preload(){
    this.length=preload.arguments.length;
    for (var i=0;i<this.length;i++){
        this[i]=new Image();
        this[i].src=imagedir+preload.arguments[i];
    }
}

var pics=new preload("mlt1.gif","mlt2.gif","mlt3.gif","mlt4.gif",
            "mlm1.gif","mlm2.gif","mlm3.gif","mlm4.gif",
            "mlb1.gif","mlb2.gif","mlb3.gif","mlb4.gif",
            "mlblnk.gif",
            "mlleft.gif","mlright.gif",
            "../buttons/edit.gif","../buttons/edit2.gif",
            "../buttons/solve.gif","../buttons/solve2.gif",
            "../buttons/play.gif","../buttons/pause.gif",
            "../buttons/mix.gif","../buttons/reset.gif","../buttons/help.gif");

var posit = new Array ();
var blnkx;
var blnky;
function initbrd(){
    posit = new Array (0,1,2,12,4,5,6,3,4,5,6,7,8,9,10,11);
    blnkx=3;
    blnky=0;
}
initbrd();
var mode = 0;   //0=normal  1=solving scrambled  2=edit  3=solving
var seq = new Array();
var edt;

document.writeln("<table cellpadding=0 cellspacing=0 border=0><tr>");
for(var i=0;i<4;i++){
    if(i==0 || i==3) document.writeln("<td><a href='javascript:left("+i/3+");focus()'><img src='"+
            pics[13].src+"' width=40 height=40 border=0><\/a><\/td>");
    else  document.writeln("<td><\/td>");
    for(var j=0;j<4;j++){
        document.writeln("<td><a href='javascript:clicked("+i+","+j+");focus()'>"+
            "<img src='"+pics[posit[i*4+j]].src+"' height=40 width=40 border=0 "+
            "name='piece"+i+""+j+"' border=0><\/a><\/td>");
    }
    if(i==0 || i==3) document.writeln("<td><a href='javascript:right("+i/3+");focus()'><img src='"+
            pics[14].src+"' width=40 height=40 border=0><\/a><\/td>");
    else  document.writeln("<td><\/td>");
    document.writeln("<\/tr><tr>");
}
document.write("<td colspan=6 align=center><a href='javascript:mix();'><img src='"+pics[21].src+"' height=16 width=48 border=0><\/a>");
document.write("<a href='javascript:reset();'><img src='"+pics[22].src+"' height=16 width=48 border=0><\/a>");
document.write("<a href='javascript:edit();'><img src='"+pics[15].src+"' height=16 width=48 border=0 name='edit'><\/a>");
document.writeln("<a href='javascript:help();'><img src='"+pics[23].src+"' height=16 width=48 border=0><\/a><br>");
document.write("<a href='javascript:solve();'><img src='"+pics[17].src+"' height=16 width=48 border=0 name='solve'><\/a>");
document.writeln("<a href='javascript:solplay();'><img src='"+pics[19].src+"' height=16 width=48 border=0 name='play'><\/a>");
document.writeln("<\/td><\/tr><\/table>");

function display(){
    var c=0;
    for (var i=0;i<4;i++){
        for (var j=0;j<4;j++){
            document.images["piece"+i+""+j].src=pics[posit[c]].src;
            c++;
        }
    }

    displaybut();

    if(mode==1 && solved()){
        alert("You solved it!\nYou don't get a prize for this though!");
        mode=0;
    }
}
function displaybut(){
    if(mode==2) document.images["edit"].src=pics[16].src;
    else document.images["edit"].src=pics[15].src;
    if(mode>=3) document.images["solve"].src=pics[18].src;
    else document.images["solve"].src=pics[17].src;
    if(mode==4) document.images["play"].src=pics[20].src;
    else document.images["play"].src=pics[19].src;
}

function solved(){
    for (var i=0;i<4;i++){
        c=i;
        if(posit[c]==12)c+=4;
        if(posit[c]>3) return(false);
        d=i+12;
        if(posit[d]==12)d-=4;
        if(posit[d]!=posit[c]+8) return(false);
        d-=4;
        while(c!=d){
          if(posit[d]!=posit[c]+4) return(false);
          d-=4;
        }

    }
    return(true);
}

function left(i){
    if(mode>=3) mode=1;
    if(mode!=2){
        doleft(i);
        display();
    }
}
function doleft(i){
    var c=i*12;
    var d=posit[c];
    posit[c]=posit[c+1];
    posit[c+1]=posit[c+2];
    posit[c+2]=posit[c+3];
    posit[c+3]=d;
    if(blnky==i*3) blnkx=(blnkx-1)&3;
}
function right(i){
    if(mode>=3) mode=1;
    if(mode!=2){
        doright(i);
        display();
    }
}
function doright(i){
    var c=i*12;
    var d=posit[c+3];
    posit[c+3]=posit[c+2];
    posit[c+2]=posit[c+1];
    posit[c+1]=posit[c];
    posit[c]=d;
    if(blnky==i*3) blnkx=(blnkx+1)&3;
}


function mix(){
    clearbrd();
    var pcs=new Array(0,1,2,3,4,5,6,7,4,5,6,8,9,10,11,12);
    for(var i=0;i<4;i++){
        for(var j=0;j<4;j++){
            k=Math.floor(Math.random()*pcs.length);
            posit[i*4+j]=pcs[k];
            if(pcs[k]==12){ blnky=i; blnkx=j; }
            pcs[k]=pcs[pcs.length-1];
            pcs.length--;
        }
    }
    mode=1;
    display();
}

function reset(){
    initbrd();
    mode=0;
    display();
}

function clearbrd(){
    //clear pieces
    for(i=0;i<4;i++)
        for(var j=0;j<4;j++)
            posit[i*4+j]=12;
}
function edit(){
    var i;
    if(mode!=2) mode=2;
    clearbrd();
    edt=0;   //first piece to be placed
    display();
}

function clicked(y,x){
    if(mode==2){  //editing
        var c=y*4+x;
        if(posit[c]==12){
            var tmp= new Array(0,4,4,8,1,5,5,9,2,6,6,10,3,7,11);
            posit[c]=tmp[edt];
            edt++;
            if(edt>=15){ //filled all but one
                mode=1;
                //find blank
                var c=0;
                for(blnky=0;blnky<4;blnky++){
                    for(blnkx=0;blnkx<4;blnkx++){
                        if(posit[c]==12) break;
                        c++;
                    }
                    if(blnkx<4) break;
                }
            }
            display();
        }
    }else if(mode!=4 && x==blnkx && y!=blnky){
        domove(y,x);
        display();
    }

}
function domove(y,x){
    //try up/down move
    var c=blnky*4+blnkx;
    while(blnky<y){
        posit[c]=posit[c+4];
        c+=4;
        blnky++;
    }
    while(blnky>y){
        posit[c]=posit[c-4];
        c-=4;
        blnky--;
    }
    posit[c]=12;
}



//Play back solution
var soltimer;
function solplay(){
    if(mode==4){
        // stop the play in progress
        clearTimeout(soltimer);
        mode=3;
        displaybut();
    }else if(mode!=2){
        // start play
        solve();
        if(mode==3){
            mode=4;
            soltimer=setTimeout("playstep()", 200);
            displaybut();
        }
    }
}
function playstep(){
    if(mode>=3){
        mode=4;
        solve();
        if(mode>=3) soltimer=setTimeout("playstep()", 200);
    }else{
        displaybut();
    }
}


function solve(){
    if(mode==0||mode==1){
        mode=3;
        seq.length=0;

        //no solution set up yet. Construct it!
        //save pieces;
        var back = new Array();
        for(var i=0;i<16;i++) back[i]=posit[i];
        back[16]=blnkx; back[17]=blnky;

        //solve first column
        if(posit[0]!=0 || posit[4]!=4 || posit[8]!=4 || posit[12]!=8){
            solvetile(8,0);
            solvetile(4,0);
            solvetile(4,0);
            solvetile(0,0);
        }
        //solve second column
        if(posit[1]!=1 || posit[5]!=5 || posit[9]!=5 || posit[13]!=9){
            solvetile(9,1);
            solvetile(5,1);
            solvetile(5,1);
            solvetile(1,1);
        }

        //solve bottom tile of third column
        push(-blnky) //blank to top
        if(blnkx==2) push(1,4,-1,6); //blank to top right
        var t=find(10);
        if(t==2) { push(4,1,6,-1,6,1,4,-1); t+=4;}
        while(t==6 || t==10) { push(7,3,5,4,-3,6); t+=4;}
        while(t==7 || t==11) { push(4,3,6,7,-3,5); t+=4;}

        //solve bottom of last column.
        var t=find(11);
        if(t==2) { push(4,1,6,-1,6,1,4,-1); t+=4;}
        while(t==6 || t==10) { push(3,4,5,-3,6,7); t+=4;}
        while(t==7 || t==11) { push(4,5,3,6,7,-3); t+=4;}

        //solve middle of last column.
        var t=find(7);
        if(t==6) { push(4,1,6,-1,6,1,4,-1); t=7; }
        if(t==7) push(3,5,-3, 6,1,4,-1,4,3,6,7,-3);
        if(t==10) {push(4,5,3,7,6,6,-3,4,1,4,-1, 6,3,4,5,-3,6,7); t=2;}
        if(t==2) push(3,4,5,-3,6, 1,6,-1,4, 3, 7,-3);

        //solve top of last column.
        var t=find(3);
        if(t==10) {push(4,5,3,7,6,6,-3,4,1,4,-1, 6,3,4,5,-3,6,7); t=2;}
        if(t==2) push(1,6,-1,4,1,4,-1,6);
        if(t==6) push(4,1,6,-1,6,1,4,-1);

        //solve top of third column.
        var t=find(2);
        if(t==10) push(4,5,3,7,6,6,-3,4,1,4,-1,6,3,4,5,-3,6,7);
        if(t==6) push(4,5,3,7,6,-3,4,1,6,-1, 6,3,4,4,5,-3,6,7);

        //restore pieces;
        for(var i=0;i<16;i++) posit[i]=back[i];
        blnkx=back[16]; blnky=back[17];
    }

    if(mode>=3){
        //do next move of prerecorded sequence
        if(seq.length==0) mode=0;
        else{
            // var c=seq.shift();
            var c=seq[0];
            for(var i=1;i<seq.length;i++) seq[i-1]=seq[i];
            seq.length--;
            if(seq.length==0) mode=0;
            if(c==4 || c==5) doleft(c-4);
            else if(c==6 || c==7) doright(c-6);
            else domove(blnky+c,blnkx);
        }
        display();
    }
}
function push(){
    //adds move sequence to movelist, and performs moves as well
    for (var i=0;i<push.arguments.length;i++){
        var m=push.arguments[i];
        if(m){
            if(m==4 || m==5) doleft(m-4);
            else if(m==6 || m==7) doright(m-6);
            else domove(blnky+m,blnkx);
            if( seq.length>0 ){
                if (m>=4 && seq[seq.length-1]==(m^2) ) seq.length--;
                else if(m<4 && seq[seq.length-1]<4) {
                    m+=seq[seq.length-1];
                    seq[seq.length-1]=m;
                    if(m==0) seq.length--;
                }
                else seq[seq.length]=m;
            } else seq[seq.length]=m;
        }
    }
}
function find(tl){
    for(var a=0;a<16;a++)
        if(posit[a]==tl) break;
    return(a);
}
function solvetile(tl,cl){
    //Solves tile tl by putting it at top of column cl, and if not last piece then moving down the column

    //gap to top. Assumes gap is not below partially solved column
    push(-blnky);

    //find tile
    for(var ty=0;ty<4;ty++){
        for(var tx=cl+1;tx<4;tx++){
            if(posit[ty*4+tx]==tl) break;
        }
        if(tx<4)break;
    }
    if(tx>=4){
        for(var tx=cl;tx>=0;tx--){
            for(var ty=3;ty>=0;ty--){
                if(posit[ty*4+tx]==tl) break;
            }
            if(ty>=0)break;
        }
    }

    if(ty==0 && cl==0){
        while(tx>0){
            tx--;
            push(4);
        }
    }else {
        if(ty==0){
            //Move tile down to second row
            if(cl==blnkx) push(6,1,4);
            else push(1); //gap to second row
            //move tile above gap
            var a=tx-blnkx; var b=a;
            while(a>0){push(4); a--;}
            while(a<0){push(6); a++;}
            //tile down
            push(-1);
            //Move top column back
            tx=blnkx;ty++;
            while(b>0){push(6); b--;}
            while(b<0){push(4); b++;}
        }else{
            //Move tile up to second row
            //Special case when lies at bottom of partially solved column
            if(ty==3 && tx==cl && tl!=8 && tl!=9){
                if(cl==0) { push(7); tx++;}
                else{
                    if(blnkx==cl) { push(6,1,4); }
                    //tile to column with blank
                    var a=blnkx-cl; while(a>0){push(7); a--;}
                    tx=blnkx;
                    //Move column upwards
                    push(3-blnky);
                    ty--;
                    //move some unsolved column down, so gap at top again
                    if(blnkx-cl>1) push(5,-3);
                    else push(7,-3);
                    //move bottom row back again
                    var a=blnkx-cl; while(a>0){push(5); a--;}
                }
            }
            while(ty>1){
                //move blank in top row over to column with the tile
                var a=tx-blnkx; var b=a;
                while(a>0){push(6); a--;}
                while(a<0){push(4); a++;}
                //move column with tile up
                push(3);
                ty--;
                //restore top row into position
                if(cl){
                    while(b>0){push(4); b--;}
                    while(b<0){push(6); b++;}
                }
                //Move adjacent column down
                if(tx==3){
                    push(5,-3);
                    if(cl || tl==0) push(7);
                }else{
                    push(7,-3);
                    if(cl || tl==0) push(5);
                }
            }
        }

        //Move tile into top of column cl. Tile is in second row now, gap somewhere in top row.

        if(cl && blnkx!=cl){
            //move gap at spot of tile now at top of column cl.
             //gap to second row
            if(blnkx==tx){
                if(tx==3)push(4,1,6); else push(6,1,4);
            }else push(1);
            //columns tile above gap
            var a=cl; while(a<blnkx){push(6); a++;}
            //pull down piece at top of column cl
            push(-1);
        }
        //move top row so that the blank lies above the tile
        while(blnkx<tx) push(6);
        while(blnkx>tx) push(4);
        //tile into top row
        push(1);
        //tile into its column
        while(cl<tx) { push(4); tx--; }
        //gap back on top row.
        push(-1);
    }

    //move down column if necessary
    if(tl>1){
        push(2); //gap to third row
        if(cl){  //move bottom row along to bring column cl tile below gap
            var a=blnkx-cl;
            while(a>0){ push(7); a--; }
        }
        push(1); //gap to bottom row
        var a=blnkx-cl; while(a>0){push(5); a--; } //gap to column
        push(-3); //column down.
    }
}
function help(){
    alert(
        "The Missing Link puzzle\n\n"+
        "The aim is to arrange the pieces so they make 4 chains, each\n"+
        "of a single colour. Press the arrows turn the top/bottom layers,\n"+
        "or press a piece to slide it up/down towards the blank space.\n"+
        "\nFurther controls:\n"+
        "Mix:    This button randomly mixes the puzzle up.\n"+
        "Reset:  Resets the puzzle to the initial position.\n"+
        "Edit:   This allows you to edit the puzzle. The pieces are\n"+
        "        cleared. The first space you click will be filled with\n"+
        "        the top red piece, the next two spaces the middle red\n"+
        "        pieces and then the bottom red piece. The other colour\n"+
        "        follow in the same way in yellow green white order.\n"+
        "Solve:  This solves the puzzle. Each time you click the but-\n"+
        "        ton one move is performed until the puzzle is solved.\n"+
        "Play:   This solves the puzzle, playing through the whole\n"+
        "        solution automatically. Press it again to pause.\n"+
        "Help:   Shows this help screen.\n"
    );
}
