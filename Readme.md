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
;:i;1:.,;1t1ii1tti;:,..,ii;;i1itttti,,,i;;::i;;;:,,::;ii1;iii111
;:1:;,.:1fffffffft1;,,,,;tttffttfft;::,:;i;:i;:;;:;::i;;1ii11i1t
:it:,.,1fffffffftfti:;:,:ifffff1iti;:;;;11;;i1iii;;:;1ii1i1t1i1t
:11,,,;fffffffttt11i;ii;i1tffff1;;:;:ittt1iiiii;:;::iiii11i1iitt
i1;,,,1fffffff1i;:;ii;;;tffffti;i1i;::i1ii11t1i1;i;:i1tttf1tiiLL
1t;:,:tfffffffti::;;;i1tfffttt1;1f11i1t11t1;1tt1ii111t11111tttCt
11:;,;1ttfffftt1t1ii;iffffttffffttfffftitf1;it1ii111ii1iit11t1ti
1i::,itfftttttffffttfffffttfLLLLffLLffft1tt;ii;:iftt1i111ti1i1i;
i:;,,1tfLfffffffLLLLLLftttfLLLLLLLLLffffttti;;::1fLLLt11tttt1t1i
:::,:tffLLLLLffLLLLLLLf1tfLLLfLLLLLft11tt1i;::::tLtLLfLffffCLfi1
::,,:ffLLLLLfffLLLLLLf11fLLLffLLLff11tttt1::::i1ff1ffttfftfCCL1t
::,,;fLLLLffffLLLLLLf1tfLLfftfttt11tLLLLf1ii;:itLt1LLfLfftff1ttt
;::,1LLLLffffLLLLLLLt1ffft11ii;;;iitfLLfft111;ifLt11fLLfL11i;11f
;,,,1ffffffffLLLLLLt1ft11tff1;,,,,:i111tftiitiii1i;i1fLffftiittf
;,.,:;i11t1ii1tfffti11i1fLf1:,.,,,.,;::itfi11;;it1iii1fLLL111fff
:,....,,,::,,,::;;::i11fff1:,:;11i,.,,,itLttti;tf1;;ifLftffftLLL
,..,,..,.,....,,,,.,;iifft;,;itLL1;,.,;11t1;;1ftf1i;1Lf1iLffttLL
...,,........,,,....,,:1ti:;1ttLLti:.,;;;;:::ift1i11tLtttf11i1CL
,..,:,,,,.,,,,,.....,,,,:::i1tftt1;:..:;;:;;:;1ii;tff11ft11t11LC
:,,::,,,,,;;:;;,....,,,,.,:itf11fi;:,.:1Lt1iiii1i;1fLtfft11ti1tL
i:::,;;,:;iii::::,,.,,,,,,:itLLttii:,,:1CL1i1tfLt1fttftft1tiiii1
i;:::;1;iii1t::::,,,,,,,,:;1tLLti:;;:itfLLti;fLftLCf1ftft1iii1ii
i;;:;itiii;;::ii:,,,,,.,,;11ttt1i::;iCCCLfff1tf1;fLftLLtt111LL1i
;:::;1fi::,,.,:;:,,,.,,,:;ii1111i:::1LLLttfffiiiit1tfLLff11tCLtf
i:iiii1i:,,...,:::...,,::::;;i1i:,,,1CLt1iii;;i;itittttff1tttftt
::1i;;;;,,,,..:;;:,,,:::;;;:::;;;,,,tCftti1;:;i:it;1tii1ffft1tfL
::;;::;:,,:;;:11;:,,:;;:;;;:,,,,,,,;Lfi11;i::iiittii1iiitttt1tft
;i11i1i;;;1t11ft1i::;:;;it1i:,,,,,;fL11ti:i;;1i1t1i11ii;ii11111i
11tt1i:;i1fLLLCCLt1i1i;1tCft1;,..,iLLt11ii;iii:;iiii111i;;iiit1i
1i1ti;:i11fLLLCCCLtiiiifC0ftti:...:fLftft1ii11ii;i1i1iiii1ti;iti
;;i1i;it11fLLLCCCCf1tfC08Gftt1;,..,tCCLffft1ti1;iit11ii111ft;:;i
iiii;1fftttLCCGGGGCG0888GfLtt1;,,.;CCfLtff1iti1;iii;;;:;;:;i;:;t
1;::i11iiiiiiiitC008888GftLCLf1;::tfftftti::;;11i;:;;;::;;11ii1t
;::;ii;;::;:,,:t08@8880CfLGGCCLLt1i;L00C1iiiiiii1i;ii;:;ii1fttft
:,::;;;ii;;:::108888800G000CCCCCL1iif0880Lt111t1tti;;;;i1i;itttt
,,,:,,:;11i::iG88888008888GCCCCCLfLLfL0888Ct111i;ii:;11iii;ittt1
;:,,,,:iiii;iL8888080888880CCCCGG0888008880L11i;;i1ii11;i1i1ffii
;::,,.,i;ii;108888080888888GGG0088888888888G11tti1t1i1iii1ii111i
;i:,,.,;;;i1f00GG0000888888888@8888888888080t1Lfi11ii;;t1tt1iitt
;i::,..:::itG000GGG00088888888@@888888880000Ctt1ittt1i1t1tft1t11
;i:,,,.,::it08000GCGG088888888@888888880G0000ttftt1t1i1ft1tttti1
:;:,,,,.,::t08800GCGC088880880888088880GG0G00ttfff1iii11i1tffttf
.,:::::,,:;L8880GLLCC088800880880080880GGGG00t11ttt;iit1;11tfftt
.,:,::,,,;;C880GCCG0000880088G000080880CGG088CfiiLfii1t1;11tf1tt
,,:;i:::;iiG0000GGGGG00G0000GLCC080G080GGG088Gftittt1111ii1ttt11
,.,iii::;itG000GCLLGGGCG0080LfLLG0000GGGG08880fttt111tt11i1ii1i;
..,:;i::i1t0800GLfLGGGGG008CttfLLGG0GLLLG88800Lfft1tiitt1111iii;
,,,:;;;:;1tfLLLfttfCG00GG0Gti1tLfLG0GftfG0800Gf1ti;i;ii;ii;;;;;;
i::;;;;;1tt11tfLfffLLCCCLLt1ii11ttf0Ct1tC0880GLffiiii1i1fi:;ii;;
i;;i;;;ifLfi;i1fffLLLffft1ii;:,:::;Lf111fLLLLLLi;::;;;;;;;:;ii;;
i;;;;:;tLCLi:;;i11tttt1iii::,,,,,,:1ttttfft1;;i:::;;;:;::::;;;:;
i;;;;:;fLCLi;,,::;;;;;;;;;:,,,,,,,,:;ii1111i::;;;;;;;;;;:,,::;:;
iiii;:1fLCL1:,,,,,:::::::::::,,,,,,,,,,:::::,:;;;;;;;;;;:,,,:::;
;;iii:1LLLLf;::::::::::;i111:..,:::,,,,,,,,,,:;;;;;iiii;::::,::;
iiii;;tffLLfi;i;;;::;i1LC0GG1,,,;;;:,,,,,,,,,;;;;;;iiiii;;;::::;
iii;:;fLLLt1::i;;ii11fG88888C1,:ii;;;::,:;:;;iii;;iiiiiii1iiii;i
;;ii:ifLCCf;,,;iii1LL0888080Cf;;iiiiii;:;1tffttt1t1ttffLCGCLCCCC
iiii:1LLLLt;,:;;1tL088888080CLtiiii1fti:;11fttttiii1tffCG8888888
iiii;1LLtt;::,:;tL08@8888G0GLLft1111LGt;ii111tf;..,,,,,:;iiii111
iiii;tftt1;:,::1L08888880G8GffLGLfLtf0L11i1ii11:,,,,,,,,..,....,
tiii;tLt11i;:;;f08888888008GffL0808CfG00C1111ii,;;:;:,,;,.,,..,;
f1;;;tf1iiiii1tC88888888880GfLL08@8GfG08Gt111i;,iiit;i::,,:,,,:1
:,,:iffi;iitttfG88888888880Ct1t08@88GGGCCf111i;,::;i:;,,.,:,,:;1
,.,:ii;;;;i1ttt008888888800Lttf0@@8880GLLLft1i:,,........,,,,:;i
```