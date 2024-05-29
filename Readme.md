## Image to ASCII

### How to build 
 
```bash
 cargo build 
 ```

### How to run 
 
```bash
 cargo run -- -i <path> -w <width> -h <height> -o <txt_path> 
 ```

### Output 
 
```bash 
;i,:11:;tfi,...,:iii11t:,::;:1f11tt:,::,,.....,:1ii:,,,::;iii;it1tttfft::,,,.,1t;;:::::;iii;;i1;:;,.,,::,::;1;ii1ti;;;iiii11iii1
i;,,;1::tt:...,,:i1ffftiiiittffffft;:;;,,..,.,,,itt1i:ii1tftii1ffffftt1:,,,,.,ii;i;;::::;;:,:;i;:,.,:,:;,,:1t;;i11;;;iiiiii11ii1
1;,:11::1;,...,,itffffffffffffffftt11ti,,,,..,,,;ifff1t1ffff1tffffffft1:,::,,,:i1i:i;,,:ii;::,::::::;:,,,:it;::;1iii;11tt1;ii1ff
1:,;tt;,:,....,itffffffffffffffffffffti,,,::,,,,,:1fffftffffffftfffffti:,;;,,,::;i;t1;::1ti;:,:ii::ii;,::;:i;,:;11ii;11ft1:i1iff
i,:1ft;,,,....;tffffffffffffffffffffft1:,,:i:,,,,:itffftfffffftii1ff1ii:;;:::..:iii1i;:;1tt1;i1tt1iii:,:;;;ftiii11iiittfii;i1i1t
,,:tfti,,....:1ttffffffffffffftttttftf1::::1i:,,:::itfffffffff1:,if1;i;:,,:11;:tftii;,::;1tiiii;;ii;:::;iiitti;;1t1;i1tfi1iit11t
,,;fti:,.,,.,ifftfffffffffLffftttt1ttti::i;11i::;i;1tfffffffffti;i;,,,:;;,;tffttffft;;1;,i1i;ii:,::::::;1t1i;;;;1iii;;i1itii111f
;:ifti,,,,,.,tffffffffffffLtttft1tt1i111;ii;ii;i1ttftffffffffft1;,,:::;1i:;1tffft1i;1tfii11ii1;:,:i1;:,:;1;:;1;i1ii1ii11;1ii1fft
1;11i:,,.,,,;fffffffffffffftii11i;;::i1t1i:::;:;tffttfffLfffti;:;;;i;;;i;::;;iti;;;;ttt1tti;;iii;;11:,,:;;it1ftttfLfii11i;;;tGCt
1;i1i,,:,,,,ifffffffffffffft1iii:,,:;;;;;1;::;;:1ffttfffffti;::;1fft1ii:,,,,;1t1;;iit1iitft1i1tti;11;;:;t1i11ffffLLf111Lt;iifGLt
iiffi,::,,,:tfffffffffffffffft1;,,,,;;:::i;i:it1tfftfffft1ii1:,:;fLf1;;i::;;tttti1ftf1;;1tfttft1;;i111iittfft1111t1111tLtfttGGft
;tff:,i;,,,;ttffffffffffffffft1;;:;;ii;;;;;1ttfffftffti;i1fffi::iffff11f11f1ft1ttfLft;::i1ffLt111iiiff1t111t1iiii1111ii1tfttGC1i
;tf1,,1:,,,;1111tffffffffffft1ii1ttii;ii::;1fffffttffiitffLLLfttfftffffffffff1;;tLff1;iiiittfi;iiiiit111ii;i111ii1tfiii1Lti1L1i;
;tfi,:i,,,,i1ttt1tfffffffft11t1tfff1i:i1;;;tfLLLftff1iffLLLLLLLLf1itLffffffff1:;tffttti;:;1t1iiiii1tfii1i;ii1t1ii;tC11ff1ii1ti;;
;tt;,;:,,,:ttfffft1ttttt111ttftffLff1;iftftffLLftfft1fLLLLLLLLLLftffLLLffLftff111itfft;,:ii11:::;tLLfi11iiii1t1iitLfi;ifti111;::
if1,,1:,,.;fttfLLfftttttttffffffLLLLffffffLLLLfftft1tfLLLLLLLLLLLLLLLLLLffftfLfti;itt1;;1111i:,::1CftfCLffi:;11i;1tiii;ii:i11;:i
1fi,:i,,,,ifttffLLLLLLfffffffffLLLLLLLLLLLLLLLftft1tfLLLLLLLLLLLLLLLLLLLLfffffLf1ii1f1i;ii;i;:,:itfLLLCCLCf1ii11i11i11t1i:tttiit
;i:,i;,,,,1ffffffLLLLLLLLLffftfLLLLLLLLLLLLLLLtttitfLLLLLLLLLLLLLLLLLLLLLftfffff1tffLft;;i:;;:,,;ttfLLfLLCLff1t11LtffffLf1ttti:t
::.,i:,,,:tLffLffLLLLLLLLLffffLLLLLLLLLLLLLLLftt;1fLLLLLLLLffLLLLLLLLLLLLtttftttfLf1t11::,:;:::,iLfLttfLLffLCffffLtfffLLCLLfi1it
i,.;;,,,,;fLffLLLLLLLLLLLLfftfLLLLLLLLLLLLLLft1;1fLLLLLLLLfffLLLLLLLLLLLtiii;;itttt;;;i:,,::::::1CGCtitGGLfLCfLLfLLffC0GCCLf;i1t
i,,i:,,,,:fLffLLLLLLLLLLLLfffLLLLLLLLLLLLLLLt;;1fLLLLLLLLffLLLLLLLLLLLf1;i1tt111iii;:,:,,::;;iiifLLf1ifLftttfttffLL11LCLCGGLiiit
,.:;,,,,,;fLffLLLLLLLLLLLfftfLLLLLLLLLLLLLLfi:1fLLLLLLLLffLLLLLLLLLLf1;itfLLLLLLLft1;:,,;;:,;f11Lft1i1LffttttttttL11fLCLCCCLtt1f
.,;,,,,,,1LLfLLLLLLLLLfffttfLLLLLLLLLLLLLLf1;tfLLLLLLLLfffLLLLLLfff1i;1fLLLLLLLLLftii;:i;,,,;iifLft1i1CCLLf11LLtffttLCCCL1tfff1t
,ii,,,..:tLLfLLLLLLLLfffttfLLLLLLLLLLLLLLf1;tfLLLLLLLLfttfftt1111i;;1ffLLLLLLLLLft1itt1ti:,::1LLCffti1fCfLLLLCLLLCL1tLt1i;11i1it
,1i,,:,.:fLLLLLLLLLLftttffLLLLLLLLLLLLLLLt;1fLLLLLLfft1;;;;::;;;iiii11tfLLLLLLLftfft1tttti;:;LLLLGLf1ti1tCLCGCLLCLtii1i;;;1i1i1L
:i:,,;:,;fLLLLLLLLffttffLLLLLLLLLLLLLLLLf;ifLLLLff1i;;i111iii;;:;:;1iiitffLLLLLLLLLt;;1tf1i::ittLCt1iiiitLLtLLtfGL11ii;;:i11ttLL
;i,,,:,,1LLLLLLffttffffLLLLLLLLLLLLLLLLfi;fLLLf1i;;i1ffLfti;:,,,,,,::iitti1fft1fLLt1i:;tft1i;;;;111i:iii1fLtGCttff11f1ii;1f1t1ff
;;,,,..,1fffffttfffftfLLLLLLLLLLLLLLLLf1;1LLf1;:i1ffLLLfti:,,.......,::1t1;;iiitLfi11;;1tt11ii;:;iii:iiiitttCLLfffLLL1;1iif1ttft
ii:,,..,;;;ii1fffffttfftt1ffLLLLffLLLf1;1fft;:itfLLLLLft;,,...........,;i;::::i1tftf1:ii1i:;iii111iiiii11iiifftfLCGL1iiii1fttfft
;;:,...,,,,,:;iiii1tti;:;;ii1tfttffLf1:;ii;::i1tLLLLLf1;,,,...,,.,,....,,;:,,,:iitLL1;1tti:::;;itL11i;i;;;1tfLLCCLCL1i1t11fLCCft
,;:....,....,:,:,,:;;,,,,,,,,,:;ii11;,,,::;1itfttLLLf1;,,.,,,:;ii1i:,. ..,,..:;iffCGfttfft1::;itCC11i:;::;tLLLtLLftft1LCf1tGGfLG
,:,..........,,,,,,,,,,,.,.,,,,,:::,,.,i1tff1tLfttLLf;,.,,,;iitLffti;. ..,,..:11ffLCLti1Lf1i;itLL1i1i:,;i1fLLf1t11fLLfLLf1fGLtLC
.,,....,,,,...,,..,,........,..,,,,...,:;111itLLftff1:,,:;ii11LGCfti;:....,.,:ift1fLft;ii;;ttttff1ii1i;:itLLLt1ti;LCLtffft1LLfCL
.,....,,.,.....,,.....,....,,.,..,,.....,:;;:1LLLf1i:,,:;i1i1fLCCft1i;:.  .,:i1tt1tti::;::;LCf1tLLfi;;ii1fGGf1ti;iCGLfffti1fLLCf
.,....,,.,,.....,...,.....,,,,,.........,,,,,;ffLf1i,,:;1tt1tfCCLLf1i;:,  .:;i;;;;;:,::;,:1tffttttt;;1iiifLCf1f1;1CL11111;iLCCCf
.,.......,......,..,,.......,,,..........,,,,,;i1t1;::;itttttttfLLft1i:,.  ,:;:::::;,,::,:;;1Lfi1;;:iLL1ifffttfftLf1;;iii;;tCCCL
.......:::,.,.,,..........,..,..........,,,,..,:::;;::;i1tttLLt1tft11i:,.  ,:;i:::,;;::i:::ift1iii::;CLttttt1tfftf1itii1i1itLCGC
,,...,::::,...::,...,,,..,::,,..........,,,,,,,,,.,:,:ii1tttfLt1tf1i;;:,.  ,::ii:;,:i;;i:::11::ii;::1CttCL1;;tfftii11tLL1ii1LCCC
,,..,,:;:,,,,,,:,,,,;i::,:ii1:..........,,.,,,.....,,:ii1tf1i;;1fL1;::,,,..,,:1ffL1i1i1i;:;1i:;ti:,;fftLGG11ttLLf1tt11tfi;1tLfLG
:;,,::,;:,,:::::,.,,:1i;,::i1,,,..,,....,:,,,,..,..,,:iitfLf1;;tff1;iti:,,.,,,ifGGC111i1i1;iii1f1::;tiiCCfifCtLCLi1111i1ii;i11ff
ii,:,:,:,:::;;,,.,,;iii;;,,;,,;:,,,,...,.,.,,..,,,:,,:i1tfLCLffffLt;iti:,,,:,,;fGGGttttiitt1tfCL1;;it1iLtiiLLtfL1ittft1111;11i1t
f1::,;:,,,;i1;,,,iit1iitti:,.,:,,i;,,,,..,,.,,.,,,,,,:i1tfLCCCf11t1;;:,,,,,,:,:tCGCCtiii111tffCCLttLCLtfLtLGt1tft11tttiiii;1iiit
ti;;::,,,:i;t1::;1i1ii;1CL;.,:,.:i;,,,,..,:,.,,,.,,:;ii1tfLCCCt;::,,;;,,,,::1iitCGGCt;;:;;iLCLffffLLCGftiiLL11fLf1t1ii;;;iii;ii1
t;::,,,:;::;11;;;iiii111t1::i;,,;:,..,,,,,,,.,,.,,,:;ii1ttfLLLft1i,.:iii;;ftLLfLLLff1f1::ifCCLf1i1tCCGGf;tLt11tft1t1i;;i;tf1;;ii
t;::,;::;::;ti:ii1i1ii;;;:,1t1i;:,,.,,,,,;,...,,,,:iiii11ttffttttti,,iii:1CCCCGCCLf1tCLi:ifffLti:;1CCCCtitLLLf1tf11111111fCL1i;;
t;i1;i:,:it1Lt;ii;i;:i:,,,,;;iti:,,,,,:,,,,...,:,::1t1111tttt1;;::;::;i;:fGGCCGLCLLftfCCff11ft1;:;tLLCt11LCCLCftf1tt11ttCG0Lt1i1
i:;i:;::;iffLLti:,::::,.,..::;;;:::,,,,,.,,,,,,,,,:i11i111ttt111i:,,:;;::LGCfLGffttCLLCLCL1iiii1iitLttt11ffLCLftCtttf11LGGGCt1fL
:,,,,,;:::;ifL1;:,:,,,...,..,:;i:,,...,....,,,,,:;:::;iii111tt11t1i,,::,:fCftLLLfiiftfttL1i;iiiii11t;itfLLtLGLLtfLt;ii1tCGCt11ff
;1,,;i;:;ii;11;::::,,,...,,..,ii,,:,.......,,,,:::::::;iiii11ii:,::::,,,:fCCCLtf1:;1;iiii;::;ii;;ifi:;tffLtLffLCLLiiii1fLffff111
i1::tf11ttt1;11i;::,,,,.......::,:::,....,,,,,,:;:;::::;;;;i11ti:,.,,,,,;CGGGf1iti;:;1;::;::ii;;;fCfi;11t11ti;1LftitfLfti;ift1tt
,;,:tti;11i;;;;;:,,,,...,....,::;;;:,,,,,,::,,,:;;;;;::::::;i1tt1;,....,1CGGC1ttLt;;1fi::::;1i::;tLti;1tft11i1ittttffff1ii1fttLL
,:,;11i:::::;:::,,,,,,.,,...:11iii:,:,::::::;;::;i;;i;::::,,::::::,.,,,,iLGGLtffffiit1i;,,:11::::iti;:iift;i11;ifLLfttLf11tLLCCC
,:,,:;;;:,,::;;,,,,,,::::,,.:tft;;,,,,.,:::;1i;;;;:::i:,,,,,,,,,,,,,,,,;tCCLi;ii11;;1i:,,,i1i;i;itf1i;;;t1;ii;;1tLLtttftii1fCLfC
;:;;:;;::::;;1i,,,,::;1i1i,;ttii;;:,::::::;:;::::::;;i;,,,,,,,,,,,,,,,:tLCLti;1tt;:;t1:,,;1ii;1tfLLft11ii11111iii1ti1ftti11tLi;1
;::i1t1;;;1111:::::;11f11titLL1iiii;;:,,:;;:ii::iiii;i1:,,,,,,,,,,,,:;1LLLf1iifLt:::i1:::if1ii1fLCtftitt111;i;;:ii;1tt1tt11i1i;:
;;;1ftt111tti:,ii;;ift11iii1tffttii;;,,,;i:,:;i;ifffLtt1:,,,,,,,,,,,:1LCft1ii1ffi;:,:i;:;i1i;;ii11iii;;1iii;i;;;;;;i1;;11iii1;i1
t11tttff1ii1;,,ii;:ittffLLLLCCCCCf1tt;:;1ti:::11itLGLtttti;,...,,,,,:tCCCL11ttiiiii::;iiiii;:::i111;;;1i111itt;;;::;;;;ti:1L1;i1
t11111tf1;ii:,::;iifCCCCCCCCCCCCCCffLftt11t1;:1ft1G0fttttt1i,.......,fCCCf1tt1;i1i11;iiiiii;:::;i1i1t1i;tfi1tti1;:;;ii;1;;tCLii;
t11t11ff1;;:,:1i1t1tLCLLLLLLCCCCCCCfff1;;ii1;;1Ltf80fttttt11;,.......iLCCfft111ft1iiiii;ii1ii:i;;;11ttii1tii11ii;;i111;:::;fLf;;
1i;1111iii;,,;1t11i1tLffLLfLCCCCCCCCfi;;:;;;iifCC080fttttt1i;,,,.....,1LCCCLt1fLf1t1i1i1111tt11i::i;1ti1iti;;;;;;11itLtii::1ttii
ii;:;;it11;,:i1tiiittLLLLLtLLCCCCCCLL1::;t1ifG008880Lttttt1i;:,,.....,;fCCCCffLLfft1i1;itfi;t1:::;;it11tttii1it1;111fCLti;::;ii;
:;::;itfi;;;tttft11f1fLLfLLLCCCCCCCCLft1LCLC088888GCLtt11t11i;,,,....,ifGGGCCCft1fLLff;1tf;ift:;11i1ff1tf1i1t1tLt11tLLfi;;:,,ii1
::;i1i1i:,;1ff1tttt1tLCCCfLGGGGGGGGGCLCCG00888888CtfCft1111ii;,,,....;L0000CGCftfCGGCLtffLi;i;:itt1i11ii;:;;;:itt;::;ii;::::,:if
11i11;;::;ttfCCLftffftL00GLG80000880GCG0888888880fitCCLt111i;:,,,,..,f88Gt1i1f1tfftti:::if1it1;;;i;i1:::::;:,:::;;,,,:11;i;:ittL
t1ii:,,:;1fttff1tt111;;1t11t11tfLCGGG0888@888880C1itLCCLLft1i;:,,...iGGCL1;:1t;fCfi1:..,;i:;tt11iii;;::;;;;:::,,::;:;t1ii1i:iLtt
1i;:,,:ii1t1;,,,:;::;i:,,,:;,,;tCG08888@8@88800Cf11tLCCCCCLLft1i:,,;LLiiLCftCCi1f1;i:,;ii::;ii1tti;::;:;;;;::::;;;;itLttiiii;1ti
;;:,,;;iiiii;::::::,:;;,,,,,,,:f08@@@@@8888880Gft11fGCCCCCCCCCCLfiift;::f8000GL1i;:ii:i1i;;;;ii11i1;:;;;i1i;,:;;ii;;1tLCft1t1fff
i;:,:i;;;;i;;;;;;::,;;:,,,,,,:1G8@@@@888880880CffftG80GCCCCCLLCCCfft;;;;f088880GLi;tti111ii11i;iit1i:i11i1t:,:i1tt1;;iff1tfLfCti
i,::::;:;;;::;iiii::;;;;,,,,:1G8888@@888880880CCGGG880GCCCCCCCCCCCt1;i;;1G8888880CfLfttt1;it111i1LCfi1i1i;:,;iii;tL1;:11iLCt1tLt
,,,,,::;;:;i;i;;i1ii;:;;:,,:iC88888888888008880888888CLCCCCCCCCGCf1ii1iiitL0888888GCt1i11i111Lfiitff1i;;:::::i;:;1i:::;1tL1ittti
.,,,,::,,,::;:;i1i11i::::::;L888888888888008888888880CLCCCCCCCCGCf11tLttt1tC88888880Cfi1t1t1i11i;;;ii::;:if1i;;i11i;::;1tii1ft11
.,,,,:;:,,,.,:;it1i11;:,:::t0888888888888008888888888CGGCCCCCCCGLffLG0CGGffCG88888880Ct11it111;;:::it:::;i1Lttiii1i11;:1ftLff111
:;:,,,:,,,..,;ii1ii11;::ii1G8888888800888088888888888GLGGCCCCCGGCG08888880G0C088888880f11i11i;;:,:;it1;;;;ifft;;ii1t1i1fffGft;:;
i;;::,,,::,.,:ii;:ii1i;;i1L888888888008880888888888880LLGCCCCCG008888888888888888888880f1it1iii;:;1111t1i;it1;;;;i111;ifLfC1i;;i
;;;:::,,:,..,,i1;:i11i;::tG888888888000800888888888888GLCCGGGG0088888888888888888888888G11t1ttt1ii1i1tfii1tf1i;;;;i11:;ittfi1iii
;;;,,:,,,....:;1;::iii:;:f08008888880008008888888888888GCG00088888888888888888888808880G1;i1tLLf1;11fffi;111;;itt1111;;;i11itt1i
;i;;:,,.,,...::;ii;iiiii;C000GG0G088000800888888888888880088@@@8888888888888888888088800f;i1LCLt1;tt111i1i;;:i1tii1ft1ii;iiitLti
;iif;,,,:,...,;;;i;ii11ttG0000GGGC0000080088888888888888888@@@@888888888888@888888088000Ci;ifCLt;i1t1ii;ii;:;itL1;ifLfftiii;i1Cf
::it;:;;,,....;;::::i1tfC00GG00CGGCGGG080008888888888888888@@@@888888888888888888008080801;1ftti;i1tt1ii;ii;;1LfiiitCLt1i1tft1ft
,iii,,;::,....,,:;::iitfG888G00GCGCCCCG0GG00888888888888888@@@@888888888888888888G0008G88Ctff1t1ittfttLL11i;itf1itffLL111fLLt;;i
,i11:,,..,,..,.,::;:iiifG8888G00GG0GCLCGCG008888888088888888@@@888888888888888080C0080G080ftftt1itt111fL11iiiftfLt1iittf11ff1;if
:;;;:,,.,,,.,,..,,,:;iiL008880008000GCCGCG000888888008888808@@888888888888888800GG008CG08G1itfLLtfL1ii1tiiiiitLfCti1i1fL1tt1iiit
:;ii:,,:,,,.,....,::::iC00888800080GGGGGCG0008888880008888G8@8008008888888888800GG000C0080fi1fLCttLtti;ii;ii;1f1t;1t1LLtfLtt1tfL
,:;;;:::,,:,,,...,:::,;C8888888800GLfLGGLC0008888880000880G0@88880008888888800GGCGG0CG0080ti1LfLfLLLL1ii;;i;it1;;:;i1LLfLftttffL
.,,,;;:;:,:;;:,..,::;:iG888888880GLfffLGLCG008888880008880G0@88800008800888800GGCGG0L000@0tiititLLtffti::ii:;1fi;:1t1fttLLLft11f
...,::::::::;::..::;i;f088888880GCCCCCCGCG0088888880808888G088880G008800888000CCCG0GC008@8f1ti;1tt1tti;:;t1itLLi:;ft11;1LfCt1t1i
...,:,,,::::,,,..:;:;;f0888880GGCCCCCG00G00088888800088888GG888800008800088800CCC08CG808@8CtLt1i;ifCLti;;1ttfCfi;;iftiitf1ti1fL1
.,.,;:,,;::::,..,:i;,:L008880GCCCCCCG00000000088880GG08888CG088000088000088880CCG80C0808@80LLf1i;1LGCL1;;;ii11iti:;1titLLt11fftt
...,::;;i::,,:,,;;;;:iC0880GGGGGGGGG00G0G000000000GGG08000LLCGGG008880GG088800GGG0GG800@@80Ltft1;ifLfffi11iiiitfi:;i11ffffttfL11
:,..,:ii1ti::;;;111i;1G8800008888000GLG0GGG00GGCCG000088GLfLCCLL008880GG088000GGG0C0808@@88GLLf1i;i11fttLft11i1fiit111tt1ff1i1ii
,;,..,;ii1i;:;::;;ii1fG0GGGG0000GGCLffG0GCGGCCCG00000888CfffLLtfG88880G0000GGGGGGGG8888@@880Lt1i1tt1i11tii1ff1ii1;iii1i;;1ti;;:;
.,..,,;11;ii:,,::;ii11G000000GGGCfttffCGGGGLCGG080008880LtffffLLLG088GG080GGCCCCGC08888@@800Lttffftt1iitiitfLftifti;it1;;tt1i;:i
.....,:iiiii;,,:;111if0888888800GLfffLCG0GCCGGGG0088888Gf1tLLLLLLfG80GG80CLCCLLLGG8888@@8G00Ctfffttftitfti11tfCt1t1i1ttiifiii;:;
..,..,,;:;;i;:,::1t11L08888000GCLLfffLCGGGC0000G008008Gf111ffLffLfL00C080GLtfLfLG08888@8GG00CftfLf1ti1Lt;;;i1ffi;tffftt1ii;i1;:;
...:,,,::i;;;;;:,;11tLCCCCGGCCLftttttLCCGGG0000GG00000L1iii1tffLCLfCGC000GCt1tffG00888800880Ci;11f1;::1i::;i11;:;1ii;;;;::;;;;;;
,,.,:,,::i;;;::::;it1111ttfLLffttt1;iLLLCG00G0GGGG00GLt1ii;1ttLCL1tLGC880GLt11tfCG0000000GGCLi1tff1;i;i;;;i1i::;ii;:,,,:i1;;::;;
;i,.,::;::;i;::iiiit1t1i1tffLLLLfffttffLLGG0GGGGCCCCft111ii1fttttfLfLG880Ct111ttLG88888880GCLtfLLLt;1i1i;;iti;;tLft:,::;11i1i;i1
it;,:iiii:;i;:;i1ttfffiii1ttffLLLLLLffffLLLCLLLLfffttt1tt;;;1i;1tftiiL80Cf111tt1fG8888888000GfffLL1;111i11tftiiCGtt1:ii;iiiti;:i
i1i:;;;i;;:;i;;ifffLLfi;ii11ttfLLLLLLLLLLLLfffffttt111111i;:,,,;;::,,1CLft1i111itCLLCGGGGG000f;;;;::::::;;;;;::;i;;i:;i;iiitii;;
;1i;;;ii;::;i:;tffLLLfi::;;ii1ttffffLLLLLLLfffft11ii;;i;::;:...,,:,,,;ffft11111itffffft111fLCf::::::::::::::;;:::::,,,:;i;;i;i;i
i11;;;;i;:,::;iffLCCCLi:,::;ii111tttttfffftttt11iii;::::,,,,,,,,,;:,,;tttttttttffLLLftt1ii;:i1:::::;;;:::::::;;:::::,:;;i:;;:;i;
iii::::;;:,::;1ffLCCCLi;:::;;;iiii11111111111iiiiiii;:,,,,,,.,,,,,,,,;111tttttfffffftt1i;;:,:;:::::;;;;;;;;;;;;;;:::,;;:;:i:::;;
1ii;:;:;i:,:,ittfLCCCL1;;:,,,:::;;;iiiiiii;;;;;;;;;i;:,,,,,,,,,,,,,,,:;:;i11111ttttt11i;:,,::;:::;;;;;;;;;;;;;;;;::,,::,::;;:,i1
i1i;;i;1;i;,,tffLCCCCL1;i;:,..,,::::;;;::::::::::::::,,::,,,,,,,,,,,,,,,,::;;iii11i1iii;:,,:;;;;;;;;;;;;;;;;;;;;;:,,,,:,::;;:::i
;i1;iii1;i;,:fLLLLCCLft;;;:,,..,,,,,,,,,:::::::::::::::::,,,,,,,,,,,,,,,,,,,,::;;;;;;;:::,,:;;;;;;;;;;;;;;;;;;;;;:,,,,,::;;:::;;
;;i;ii;i;:,,;fLLfLCCLffi:,,,,..,,,,,,,:::::::::::::::;;;;,.....,:::,,,,,,,,,,,,,,:::,,,,,,,:;;;;;;;;;;;;;;;;;;;;;,,,,,,,,,;::;ii
i;;;ii;i;i,,ifLffLLLLLLt:,,,,,,,,,,::::::::::::::;ii;iii;......,;;;:,,,,,,,,,,,,,,,,,,,,,,,:;;;;;;;;;;;iiiiiiii;;,,,:;;:,,,::;;;
i;;iii;i1i,,iLLLLLLLLCLf1:::::::::::::::::::::;;i1tf1i11i,.....::::;::,,,,,,,,,,,,,,,,,,,,,;;;;;;;;;;;iiiiiiiii;;:::;;;,:,::::;;
ii;iiiiii;:;tLLfffLfLCLt1iiiii;;;;:::::::;;;ii1tfC00CCGGf,,,,..:;;;;;::,,,,,,,,,,,,,,,,,,,:;;;;;;;;;iiiiiiiiiiiii;;ii;:,,:::::ii
iiiiiiii;:,;tLLftLLfffff1;;i1i;;;;;;;;;;;iii1fG088888888Ci:,,,,:;;;i;;;:::,,,,,,,,,,,,,,,,;;ii;;:::;iiiiiiiiiiiiiiiiii::;;;;:;;;
iii1i;;;:,,;tfLffLLLftt1;:::i1i;:;;;;;;iii1tC08888808888GLt:,,,;ii;i;;;;;::,,,,,,,,,,,,,,;iiiii;:::;iiiiiiiiiiiiiiiiiiiiiii;;:;i
;iiiiii:;:,;fLfLCCCLf11i:,,:i1i;;iiiiiitt1fC088888808888CCLi,,,;iiiii;ii;;;;::::::;i;;:;iiiiiiii;;;iiiiiiiiiiiiiiiiiiiiiiiiii;ii
;;i;iii;;::iLLLCCCCCfti;:,,,:;i;;iiiiifGffG08888888G8880CCLfi,,;iiiiiiiiiiii;;,:;i1ftt1CCft111ft11ff1i1111fft1tfLLLffftttffffttt
;;;;;iii;,;tLfCCCCCCLf;:,..,:;11ii111tGGLG088888880G8000CCLLt:,iiiiiiiiiiiiii;,:;i1tfffGCfft1tLf11tf11ff1fGGLLC08888880008888880
i;;;iiii;,;ffLCCLLCLLf1;,,,:;;1ii111tC0G08888888880G8000CCLLfi:iiii11ii1tt1ii;,,:i1tttfLffff1tftiii1i1fftfLCfCG008@@@@@@@@@@@@@@
iiii;iii;,;fLCCLfLLfii1;,,,:i;;:i11tL0008888888888GG888GCLfLL1:i11i11ii1CGCti;::;ii1i1ft1ttttLti;;;;ii111ttt1LCGC0888888888@@@@@
iiii;iii;:itLCLLtfft;:::::,,,,:;1ttfG808@@88888888C0888GLLLLLti111i111i1tG8Gfi::;ii1i1t1ii1fLLi,.....,,,,,,,,:;;;i1tttffffLLLLLC
iii11iiii:;tLLLL1;11;:,:,:::,:;ittLG888@@88@@@@880C0800CLffLLffft111t1111C88Ci;;iiii11iiii1ffC1....................,,,,,,,,,,,,,
iii11ii;;;ifLftt11t1i;:,.,::::i1tLG8888@88@@@@@880C080GCLfffLLG0Lt1tLf111L00C1;;iiii1iiiii1ttLi.,,,,.,,.......,,...............,
iiii1iii;:1fLfttfff1i;::,,,,::itLG08888@888888@88GC8880GLfffLLG80LfL0Gf11f00Ctttt1ii1iiiiii1i1:.,,,,,,,,,,,,,,::,..,,,.........:
1i;i1iii;:1LLLt11ttt;;;;:,,:;;iL08888888888888888GG8888GftffffG880GG88Gt1fG00088Gf1i1ii111iiii,.,;;,,,,:,...,,ii:....,,........1
Lf1iiiiiiittLLfiii111iiii;;;;;iG888888@8888888888G08800CttfffLG888@8880L1tG08@@@8L111i1111iiii,.:t1:;;11;,,,.,ii;..,,.,,.,...,,f
LLf1ii;::iffLff11i111iiii;;i1i1088888@@888888888808880GLtfLLLLG8888@@88CttC008@@0C11111111iii;,.:t1:;iffi,it:,:i;,,:;,::.,:.,;;L
ft1i;;;:,:itLLtiiii1ii11ii1tffL8888888@@888888888888880CLLLLLCG88@@@@88GftC00080GGt111iiiiiii:,.:t1;;1Lt;,1t:,.i:..,::,,:,::::if
;:,,::;;:i1fLLt;;;;i;ittttttffC888888888888088888888888CffffffG88@@@@880CfC0GGCLCGL111iiiiiii:,.:1iii1ff1,1t:,.;,..,:::,:,,:::tt
,...,,:;;fLLffiii;;i;itftttfttG88888888@@88008888800800fi;;;;iC@8@@@@8880GG00GLfLCCtt1111iiii:,,,,,,,:;;:.:;,,.:,..::,:,.::,:ifi
,,. .,,:;ffi;::i1;iiii1ttttt1t080088888@@8800888880G00Gt11ttttC@@@@@888888000GLffLLLttt1t1ii;:,,,,.............,...::,,,,,:,:it:
,,,..;;,:i;::::::::;;;i11ttt1f0000888@@888888888880G0GCtLCLLLfC8@@@@88888880GCLLfffLffLtt11i::,,,...................,:.,,.,;:;;:
```