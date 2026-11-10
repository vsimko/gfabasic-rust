# Amiga GFA-BASIC binary analysis

Reverse-engineering evidence for GFA BASIC 3.52 only; not a verified 3.5 language specification.
Strings and function labels are candidates. Command syntax requires parser/dispatch analysis and manual verification.

- Printable strings: 3270
- Analyzed functions: 571
- Architecture: Motorola 68000, big-endian
- Instructions exported: 16733
- References exported: 6085
- String cross-references: 160
- Instructions outside known functions: 5895
- Strings with cross-references: 102
- Executable memory blocks: 1

## Printable strings (candidate evidence)

| File offset | Length | Text |
|---:|---:|---|
| 0x7C | 4 | gP"* |
| 0xE6 | 4 | ,@-\| |
| 0x11E | 5 | J4H\N |
| 0x17A | 4 | "N < |
| 0x1A9 | 4 | Hpd` |
| 0x20E | 4 | "_ x |
| 0x216 | 4 | f #y |
| 0x236 | 4 | Nu#\| |
| 0x256 | 6 | Nua:pl |
| 0x2E2 | 17 | intuition.library |
| 0x2F4 | 11 | dos.library |
| 0x300 | 16 | graphics.library |
| 0x311 | 14 | layers.library |
| 0x406 | 5 | DE`4~ |
| 0x41C | 5 | #\DB` |
| 0x442 | 5 | ghDBH |
| 0x44A | 5 | gXDE~ |
| 0x469 | 4 | Gb8( |
| 0x47C | 5 | BCHC` |
| 0x483 | 4 | GHD8 |
| 0x4A8 | 5 | DBNu  |
| 0x4B1 | 4 | BNup |
| 0x4DE | 5 | DBNur |
| 0x4E8 | 5 | NuH@g |
| 0x504 | 5 | DBNu0 |
| 0x524 | 11 | DBNuk(g.JBj |
| 0x53A | 5 | SBDBj |
| 0x5B5 | 6 | QRBNua |
| 0x5C2 | 5 | DBNug |
| 0x5CE | 9 | DE` DBDEa |
| 0x5D8 | 5 | DBNug |
| 0x5E2 | 5 | DBNut |
| 0x62F | 5 | @B@H@ |
| 0x648 | 5 | NuJBj |
| 0x652 | 4 | Nu~( |
| 0x675 | 4 | EJEo |
| 0x68C | 5 | DEDBH |
| 0x6A6 | 5 | HEJDg |
| 0x763 | 4 | QRBJ |
| 0x76A | 5 | DBNut |
| 0x770 | 4 | Nu?< |
| 0x833 | 4 | hSF` |
| 0x8AA | 5 | SFk6a |
| 0x8C5 | 4 | 4FFa |
| 0x8E8 | 5 | DBJ.# |
| 0x91E | 5 | v JBl |
| 0x924 | 4 | DBv- |
| 0x97B | 4 | QRB` |
| 0x9BD | 4 | Ev+0 |
| 0x9C6 | 4 | DFv- |
| 0x9D8 | 4 | BFHF |
| 0x9FE | 6 | HQ2.$, |
| 0xA4A | 5 | "_Nua |
| 0xA70 | 4 | JAgB |
| 0xA7E | 5 | mHJ@j |
| 0xA8E | 5 | DBUBk |
| 0xAC4 | 4 | "_Nu |
| 0xAFC | 7 | =@$,Nua |
| 0xB0E | 4 | DBNu |
| 0xB32 | 4 | Nu < |
| 0xB49 | 4 | .JBg |
| 0xBCB | 6 | PH@Nur |
| 0xC44 | 12 | H@NuH@RBNu4< |
| 0xC5C | 4 | g8k2 |
| 0xC76 | 16 | H@DBNuH@RBDBNu4< |
| 0xC8E | 6 | DBNu4< |
| 0xCA6 | 4 | jPDB |
| 0xCB4 | 4 | NuDE |
| 0xDC7 | 4 | Acrt |
| 0xDE2 | 4 | JBg" |
| 0xE23 | 5 | VJBg" |
| 0xEF4 | 4 | Nu~# |
| 0xF25 | 4 | 8DBa |
| 0xF60 | 4 | DB&< |
| 0xF8D | 7 | n8<ND:< |
| 0x10B6 | 5 | DBNup |
| 0x10EA | 4 | SBDB |
| 0x110D | 4 | TSWa |
| 0x112A | 4 | gG8< |
| 0x1164 | 6 | 2<I@4< |
| 0x11E7 | 4 | XNua |
| 0x11F4 | 6 | 8<7(:< |
| 0x1264 | 4 | H@Nu |
| 0x126A | 4 | Nu < |
| 0x12AA | 4 | DBBg |
| 0x12DA | 4 | e>&< |
| 0x1303 | 5 | tDB&< |
| 0x1317 | 4 | (RWH |
| 0x1383 | 5 | ]:Z8< |
| 0x1398 | 6 | 8<>::< |
| 0x13B2 | 6 | 8<~F:< |
| 0x13E0 | 7 | g<UGk&g |
| 0x13E8 | 4 | DB&< |
| 0x13F8 | 6 | ` DB&< |
| 0x141D | 4 | "J_j |
| 0x1422 | 6 | DBNu < |
| 0x1432 | 5 | NuJBk |
| 0x143C | 4 | Nu~2 |
| 0x1458 | 5 | HSJBg |
| 0x1488 | 7 | JGfjHGH |
| 0x14F2 | 5 | k:NuH |
| 0x1580 | 5 | J.$.j |
| 0x1586 | 5 | DBNu~ |
| 0x1601 | 4 | (JBj |
| 0x1624 | 5 | ;)8<\ |
| 0x167A | 4 | #z8< |
| 0x168C | 4 | #}8< |
| 0x174C | 6 | DBB@H@ |
| 0x1756 | 7 | R@NuRBf |
| 0x177C | 6 | DBB@H@ |
| 0x178A | 5 | NuRBf |
| 0x17C6 | 5 | `LFG/ |
| 0x17EA | 4 | DB&< |
| 0x17FB | 4 | DSBH |
| 0x1806 | 5 | RBDBH |
| 0x1896 | 5 | qV8<  |
| 0x18B2 | 6 | 8<;+:< |
| 0x18CA | 8 | pC8<&o:< |
| 0x18F7 | 8 | HJ_g*J_g |
| 0x1904 | 4 | DB:< |
| 0x1917 | 4 | (J_j |
| 0x1922 | 7 | DBNuJ_g |
| 0x1940 | 5 | DBNua |
| 0x1972 | 5 | JCg~? |
| 0x19E5 | 4 | QRBJ |
| 0x19EC | 5 | DBNut |
| 0x1A10 | 5 | JDgH2 |
| 0x1A1C | 4 | HCH@ |
| 0x1A46 | 4 | HCH@ |
| 0x1A62 | 5 | HCH@: |
| 0x1A86 | 5 | HCH@: |
| 0x1AA0 | 15 | DE`<DBDEa6DBNug |
| 0x1AB0 | 9 | DBa,DBNut |
| 0x1ABA | 4 | Nu~( |
| 0x1AEB | 4 | &JDf |
| 0x1B9E | 18 | GFA-BASIC Commands |
| 0x1BF7 | 4 | h$IG |
| 0x1C64 | 5 | GFABf |
| 0x1C6B | 6 | \|ASg$N |
| 0x1C80 | 4 |  n.* |
| 0x1C87 | 6 | r n.&0 |
| 0x1CD0 | 5 | ~f n( |
| 0x1D08 | 5 | "@/.' |
| 0x1D12 | 5 |  R-H' |
| 0x1D1A | 6 | a\`D4< |
| 0x1D64 | 5 |  _-H' |
| 0x1DBD | 4 | hNuN |
| 0x1E02 | 4 | fD0. |
| 0x1E63 | 6 | Z n. 0 |
| 0x1E76 | 7 | =\|.,/"N |
| 0x1EA0 | 6 | NuOK > |
| 0x1EFA | 5 | S@NuN |
| 0x275F | 4 | C p0 |
| 0x277F | 4 | G {p |
| 0x278D | 4 | G {p |
| 0x2B98 | 5 | +vpBN |
| 0x2BA0 | 4 | NuHy |
| 0x2BAF | 4 | G {p |
| 0x2F34 | 4 | 'nHy |
| 0x2F43 | 4 | G {p |
| 0x3357 | 4 | G {p |
| 0x356A | 5 | NuB./ |
| 0x35AC | 5 | (@Nup |
| 0x35CC | 5 | DBNu~ |
| 0x361C | 5 | Nu .' |
| 0x3628 | 6 | g  @0( |
| 0x364A | 5 | Nu .' |
| 0x3658 | 4 |  @0( |
| 0x3680 | 4 | R@29 |
| 0x3696 | 5 | X@Nua |
| 0x36B8 | 5 |  _Nua |
| 0x36C6 | 5 | Nup@N |
| 0x36D6 | 4 | Nu y |
| 0x36E3 | 4 | HNua |
| 0x3741 | 4 | pBg` |
| 0x374D | 4 | pBga |
| 0x3791 | 4 | DNua |
| 0x37C5 | 6 | DNuar/ |
| 0x37E7 | 6 | CNuaP/ |
| 0x3803 | 6 | CNua4/ |
| 0x381B | 4 | BNua |
| 0x3843 | 4 | G {p |
| 0x3851 | 4 | G {p |
| 0x3C93 | 4 | 6Nua |
| 0x3CC9 | 6 | t$@ @J |
| 0x3D09 | 4 | 8 _  |
| 0x3D2F | 4 | H @a |
| 0x3D35 | 4 | n _` |
| 0x3D42 | 5 | Nu _r |
| 0x3DA4 | 4 | $H-O |
| 0x3E56 | 7 | B@H@Nua |
| 0x3E8E | 4 | H@B@ |
| 0x3EC3 | 5 | AH@B@ |
| 0x3F56 | 4 | jHDB |
| 0x3F60 | 5 | NuRBf |
| 0x3F6C | 5 | NuRBf |
| 0x3F74 | 4 | NuDB |
| 0x3FD6 | 4 | Nu . |
| 0x408A | 5 | Nu$H$ |
| 0x40FC | 4 | # #  |
| 0x4104 | 8 | # # # #  |
| 0x415C | 11 | d~# # # # A |
| 0x41E4 | 5 | Nu$H$ |
| 0x433C | 4 | NuHi |
| 0x434E | 6 |  <0000 |
| 0x437A | 9 | NuaJJ./;k |
| 0x43BC | 4 |  0 0 |
| 0x4407 | 5 | CRANu |
| 0x4431 | 6 | ./;kXt |
| 0x445C | 5 | S@f.` |
| 0x44DE | 7 | SAkJSAk |
| 0x44F7 | 7 | BHCSCk. |
| 0x4574 | 5 | JANuS |
| 0x457A | 7 | R@JANut |
| 0x4598 | 5 | Nut;a |
| 0x45F0 | 6 | /;2<., |
| 0x45FB | 9 | Y=A/"Nu2( |
| 0x460B | 5 | @g4RA |
| 0x4642 | 4 | Nu1@ |
| 0x4686 | 5 | gJ  k |
| 0x468C | 5 | &@ S` |
| 0x4696 | 5 | g6  k |
| 0x469E | 4 | &@0+ |
| 0x46BC | 9 | # # # # Q |
| 0x46DC | 4 | J@g> |
| 0x46E8 | 4 | J@g2 |
| 0x46F0 | 5 | `: .' |
| 0x4765 | 4 | hDBj |
| 0x48B0 | 4 | NuRB |
| 0x48E4 | 7 |  @HP"Xp |
| 0x48F2 | 4 | ~ RA |
| 0x492A | 4 |  W P |
| 0x4954 | 5 |  _`J  |
| 0x4974 | 4 |  W2( |
| 0x49A0 | 5 |  _"X2 |
| 0x49EC | 4 |  W4( |
| 0x4A0A | 5 |  W PA |
| 0x4A25 | 13 | T @"_HSHPHQ0( |
| 0x4AF6 | 4 | Nu~< |
| 0x4B04 | 6 | g@`Z~< |
| 0x4B14 | 6 | f0`J~< |
| 0x4B20 | 10 | NuaDe `:~< |
| 0x4B30 | 5 | Nua4c |
| 0x4B36 | 4 | `*~< |
| 0x4B40 | 5 | Nua$c |
| 0x4B73 | 4 | 0$I> |
| 0x4B93 | 4 | ANua |
| 0x4B9A | 4 |  @2( |
| 0x4BAA | 5 |  P"H` |
| 0x4BC0 | 7 |  _ PSAv |
| 0x4C36 | 4 | DBNu |
| 0x4CB4 | 5 | Nua*/ |
| 0x4D0A | 4 |  @4( |
| 0x4D25 | 4 | &$_/ |
| 0x4D57 | 7 | tHn*&Hn |
| 0x4D64 | 4 | Nu"< |
| 0x4D77 | 5 | \&@ o |
| 0x4DBB | 4 | d$@J |
| 0x4E17 | 6 | ` @"_  |
| 0x4E3B | 4 | h _/ |
| 0x4E51 | 4 | @c<H |
| 0x4E58 | 6 | d,J@gB |
| 0x4E7A | 5 |  F"GQ |
| 0x4EA2 | 7 | Nua0U@k |
| 0x4EB2 | 7 | Nua Y@k |
| 0x4EDA | 5 | "@ Y0 |
| 0x4FEC | 8 | HP?.$,=h |
| 0x4FFD | 6 | &=_$,2 |
| 0x5006 | 5 | "@$Y4 |
| 0x503C | 11 | HANuHABAJBj |
| 0x5096 | 4 | DBNu |
| 0x509E | 5 | NuJBj |
| 0x50D3 | 4 | p/,H |
| 0x512A | 6 | /"r0XD |
| 0x5197 | 4 | ./"f |
| 0x51AD | 4 | ./"g |
| 0x5205 | 6 | r @"_  |
| 0x5229 | 4 | z _/ |
| 0x523F | 4 | @cFH |
| 0x524C | 6 | g,J@gH |
| 0x5272 | 5 |  F"GQ |
| 0x5298 | 5 | NuD@H |
| 0x52A2 | 5 | Nu? P |
| 0x52BC | 5 | BgHUa |
| 0x52C3 | 4 | 6>.! |
| 0x52CE | 7 | XGg:TGg |
| 0x5358 | 5 | NuapJ |
| 0x53DD | 4 | "gBp |
| 0x5414 | 4 | NuHp |
| 0x541E | 5 |  _Nup |
| 0x546E | 4 | NuHp |
| 0x5476 | 9 | U@a\| _Nup |
| 0x54CE | 5 |  _NuX |
| 0x54D8 | 5 | NuHSG |
| 0x54F2 | 7 | &_NuHPa |
| 0x551D | 4 | R$.i |
| 0x5527 | 5 | h"B#H |
| 0x5549 | 4 | hNup |
| 0x5576 | 4 | &_Nu |
| 0x55FE | 7 | NuHP&I> |
| 0x5606 | 4 | g"SG |
| 0x562C | 4 | Nu n |
| 0x5648 | 4 | Nu-H |
| 0x56A8 | 4 | RH=H |
| 0x56B6 | 4 | Nu n |
| 0x56C7 | 6 | ANu .( |
| 0x56D4 | 4 |  @ h |
| 0x56DF | 8 | &R@RANuN |
| 0x56F4 | 7 | Nu&G$.( |
| 0x5702 | 4 | "B3\| |
| 0x571C | 4 | 0000 |
| 0x5730 | 4 | 0000 |
| 0x574C | 4 | NuBn |
| 0x576D | 4 | hB0` |
| 0x5803 | 4 | ~SGa |
| 0x5813 | 4 | nRGa |
| 0x585E | 7 | gvJFfr` |
| 0x5888 | 4 | jBRG |
| 0x5890 | 4 | SG`4 |
| 0x589C | 5 | `*J./ |
| 0x58C4 | 5 | RGRF` |
| 0x5916 | 5 | RGRFC |
| 0x592B | 4 | JSWk |
| 0x593E | 5 | m p a |
| 0x5963 | 4 | Ff&E |
| 0x596A | 5 | `0JGg |
| 0x5976 | 4 | SGSF |
| 0x59A9 | 4 | GR@? |
| 0x59E1 | 4 | FJ@g |
| 0x59F8 | 5 | *_ @0 |
| 0x5BAC | 4 | NuU. |
| 0x5BEB | 4 | h"H0 |
| 0x5C36 | 7 | B NurC` |
| 0x5C44 | 7 | J@g:t0C |
| 0x5C8A | 5 |  n/$a |
| 0x5C9B | 4 | n/(c |
| 0x5CB0 | 9 | -@/$ @"X0 |
| 0x5CBA | 5 | =@/*g |
| 0x5CC2 | 4 | Bn/( |
| 0x5CE6 | 6 |  n/$"X |
| 0x5CEE | 12 | /(Rn/(Sn/*kz |
| 0x5D11 | 4 | \g.a |
| 0x5D1A | 10 | Rn/(Sn/*kP |
| 0x5D44 | 11 | R@Rn/(Sn/*k |
| 0x5D83 | 6 | h n/$a |
| 0x5DA8 | 8 | /, n/$"X |
| 0x5DB2 | 11 | /(Rn/(Sn/*k |
| 0x5DC3 | 4 | _g,v |
| 0x5DE9 | 4 | #g$a |
| 0x5DF2 | 9 | Rn/(Sn/*k |
| 0x5E14 | 5 | Sn/*k |
| 0x5E24 | 5 | Sn/*j |
| 0x5E41 | 4 | ./#g |
| 0x5E59 | 6 | ./"fXx |
| 0x5E64 | 6 | Sn/*kL |
| 0x5E85 | 4 | _f.R |
| 0x5E8A | 5 | Sn/*j |
| 0x5E98 | 5 | Sn/*k |
| 0x5EB6 | 8 | Sn/( n/$ |
| 0x5ED8 | 7 | =n/6/4j |
| 0x5EE0 | 5 | Dn/6N |
| 0x5EF6 | 5 | z* I0 |
| 0x5F1C | 5 | Jn/4j |
| 0x5F28 | 6 |  n/$ X |
| 0x5F30 | 7 | /(Rn/(0 |
| 0x5F39 | 5 | n/(c\| |
| 0x5F41 | 5 | ./"g8 |
| 0x5F4D | 5 | ./#g@ |
| 0x5F75 | 10 | ^Rn/("_p$R |
| 0x5F85 | 4 | N"_` |
| 0x5F8C | 5 | Rn/(` |
| 0x5FA6 | 5 | Jn/4j |
| 0x5FB0 | 5 | Jn/4k |
| 0x5FBA | 13 | =_/(Sn/(Rn/*` |
| 0x5FD2 | 7 | /,=B/4j |
| 0x5FF6 | 5 | RCR@r |
| 0x6061 | 6 | Er+J@j |
| 0x6068 | 4 | r-D@ |
| 0x606E | 7 | UEoJYEc |
| 0x6079 | 4 | 0SEn |
| 0x608E | 5 | SEH@H |
| 0x6097 | 4 | dREf |
| 0x60A2 | 5 | SEH@H |
| 0x60AE | 7 |  0 0REf |
| 0x60C0 | 6 |  n/$ X |
| 0x60C8 | 7 | /(Rn/(0 |
| 0x60D1 | 5 | n/(cH |
| 0x60D9 | 4 | ./"g |
| 0x60E5 | 4 | ./#g |
| 0x6104 | 5 | Rn/(` |
| 0x610A | 5 | Jn/4j |
| 0x6114 | 5 | Jn/4k |
| 0x611E | 13 | =_/(Sn/(Rn/*` |
| 0x614A | 4 |  J0< |
| 0x616C | 4 | gj=\| |
| 0x61A2 | 4 | g,$* |
| 0x61AA | 5 | a.`"E |
| 0x6217 | 4 | \|afJ |
| 0x62E6 | 4 | Nu=\| |
| 0x6416 | 5 | NuJBj |
| 0x6446 | 5 | a`JAg |
| 0x645C | 5 | NuJDf |
| 0x64AE | 5 | g<HPB |
| 0x64F1 | 4 | `HPB |
| 0x6530 | 5 | 2./<g |
| 0x6572 | 5 | HPHQS |
| 0x65E8 | 4 | HCBC |
| 0x6609 | 7 | h _HR4< |
| 0x6611 | 6 | ,a:HRt |
| 0x661C | 8 | a0HRt( o |
| 0x6626 | 5 | a&/.' |
| 0x6639 | 6 | h"HJ@g |
| 0x6664 | 8 | $_NuaDo@ |
| 0x6671 | 4 | F"6p |
| 0x6676 | 4 | g4 A |
| 0x670D | 4 | F"0p |
| 0x673C | 5 | Nu .i |
| 0x6744 | 6 | "@&@3\| |
| 0x6757 | 4 |  Nua |
| 0x676D | 6 | Vk:JBk |
| 0x6789 | 4 | F"0p |
| 0x67CB | 4 | lNua |
| 0x680F | 6 | .J@g&? |
| 0x6838 | 4 | J_Nu |
| 0x68A0 | 4 | NuaT |
| 0x68CB | 4 | pp,a |
| 0x69B8 | 17 |                 R |
| 0x6B7E | 5 | $F .i |
| 0x6B8C | 5 | UP-@i |
| 0x6B96 | 4 | &@"@ |
| 0x6BDC | 5 | Nu$.i |
| 0x6BEB | 6 | h"BpRN |
| 0x6BF5 | 4 | ~"ni |
| 0x6C0C | 15 | Nuserial.device |
| 0x6C35 | 4 | ~pdN |
| 0x6C4F | 4 | BHPa |
| 0x6C7E | 5 | COM1f |
| 0x6C9E | 4 | fX$< |
| 0x6CAA | 7 | CON:fHp |
| 0x6CC8 | 4 | HP < |
| 0x6CDB | 4 | \ _  |
| 0x6CEE | 5 | NupXN |
| 0x6D63 | 4 | JRAJ |
| 0x6D91 | 4 | JRAJ |
| 0x6DE7 | 4 | JRAJ |
| 0x6E16 | 5 | b$ _C |
| 0x6E1D | 4 | Drcv |
| 0x6E61 | 4 | Mf`a |
| 0x6E8E | 4 |  _B0 |
| 0x6EC1 | 6 | BNupc/ |
| 0x6FBB | 4 | T=F! |
| 0x6FC2 | 4 | g&&< |
| 0x6FCF | 4 | F"0` |
| 0x6FF5 | 4 | H-@/ |
| 0x6FFD | 4 | @"n/ |
| 0x7049 | 4 | B]An |
| 0x7073 | 4 | ` p8 |
| 0x70D1 | 4 | l @0 |
| 0x70DE | 5 | HP Sz |
| 0x70EB | 6 | `$_HP$ |
| 0x7102 | 9 | f8 W P"[6 |
| 0x7148 | 5 | afHPp |
| 0x7162 | 7 |  P`"$Y4 |
| 0x719C | 4 |  _"_ |
| 0x71D2 | 4 |  P ( |
| 0x71DF | 6 | ^&_ S2 |
| 0x722B | 4 | 2"K" |
| 0x72D7 | 5 | \|PRT: |
| 0x72DD | 4 | PAR: |
| 0x72E2 | 4 | SER: |
| 0x7342 | 5 | Nu$.i |
| 0x734C | 4 |  B!@ |
| 0x7355 | 5 | ("H3\| |
| 0x7375 | 4 | C p0 |
| 0x7393 | 4 | C p0 |
| 0x73AF | 4 | C p0 |
| 0x73CD | 4 | C p0 |
| 0x73EB | 4 | C p0 |
| 0x741B | 4 | C p0 |
| 0x7435 | 4 | C p0 |
| 0x7451 | 4 | C p0 |
| 0x746D | 4 | C p0 |
| 0x7485 | 4 | C/00 |
| 0x74A1 | 4 | C/00 |
| 0x74B9 | 4 | C/00 |
| 0x74C1 | 4 | X _0 |
| 0x74D1 | 4 | C/00 |
| 0x74E9 | 4 | C/00 |
| 0x74F2 | 5 |  _JBV |
| 0x750B | 4 | C 08 |
| 0x7510 | 5 | NuHp8 |
| 0x7521 | 4 | `/00 |
| 0x753A | 4 | "@0) |
| 0x7544 | 7 |  _$PS@k |
| 0x7554 | 4 | Nu2( |
| 0x755C | 4 | "PRA |
| 0x756E | 5 | "@$Y2 |
| 0x758A | 4 | NuBh |
| 0x75BF | 4 | \ p@ |
| 0x75C4 | 5 | "F`6v |
| 0x75F5 | 4 | \ q0 |
| 0x765B | 4 | C/00 |
| 0x768B | 4 | \ p0 |
| 0x769E | 4 | Nu n |
| 0x76B5 | 4 | C/00 |
| 0x76E5 | 4 | d p0 |
| 0x76F2 | 4 | Nu n |
| 0x7709 | 4 | C/00 |
| 0x7739 | 4 | \| p0 |
| 0x7746 | 4 | Nu n |
| 0x775B | 4 | C/00 |
| 0x7796 | 4 | Nu n |
| 0x77AF | 4 | C p0 |
| 0x77BC | 5 | BgHPR |
| 0x77ED | 4 | h p0 |
| 0x7812 | 4 | Nu n |
| 0x782D | 4 | C/00 |
| 0x7861 | 4 | ` p0 |
| 0x7898 | 5 | NuatR |
| 0x78A2 | 7 | :0$_ Z0 |
| 0x7906 | 5 | $_ Z0 |
| 0x790D | 4 | ANuv |
| 0x793F | 4 | ` p8 |
| 0x7958 | 4 | g<0< |
| 0x7998 | 4 | Nu". |
| 0x79BC | 7 | a$a"a a |
| 0x7A16 | 5 | HEJEg |
| 0x7A29 | 4 | bU@g |
| 0x7A2F | 4 | xS@g |
| 0x7A65 | 4 | D8;@ |
| 0x7A9D | 4 | D8;@ |
| 0x7ACD | 4 | d p0 |
| 0x7AE7 | 4 | \| p0 |
| 0x7B1F | 4 | h p0 |
| 0x7B31 | 4 | \ p0 |
| 0x7BD5 | 4 | \ p0 |
| 0x7C19 | 4 | ` p0 |
| 0x7C76 | 4 | RGNu |
| 0x7CB3 | 4 | C p0 |
| 0x7CCC | 4 | gn _ |
| 0x7CD2 | 4 | d\Nu |
| 0x7DFB | 4 | B _  |
| 0x7F27 | 4 | p`&r |
| 0x7FAA | 5 | ` RAf |
| 0x805C | 4 | Nu n |
| 0x8062 | 4 | a\ n |
| 0x806E | 7 | g42 $`g |
| 0x8076 | 4 | k.&H |
| 0x80D0 | 7 | gD" &Ak |
| 0x80EC | 5 | g(" k |
| 0x8109 | 4 | I3 Q |
| 0x8130 | 5 | g^" k |
| 0x8154 | 5 | g:" k |
| 0x8183 | 4 | I3 Q |
| 0x81B1 | 7 | B v(\ p |
| 0x81C6 | 7 | HQ !UAk |
| 0x81D6 | 4 | "_Nu |
| 0x81F1 | 7 | G$vp\"r |
| 0x81FE | 6 | gz$6p` |
| 0x8283 | 6 | XNup@N |
| 0x82A2 | 5 | gBnVa |
| 0x8315 | 4 | bHA2 |
| 0x83EA | 6 |  @"C2( |
| 0x83F6 | 5 |  P"Q: |
| 0x8413 | 4 | CJ./ |
| 0x841B | 4 | C @/ |
| 0x84DB | 6 | laLHPa |
| 0x84F3 | 6 | ta4HPa |
| 0x84FB | 4 | B _  |
| 0x8508 | 5 | a HPa |
| 0x8528 | 5 | Nu p8 |
| 0x8558 | 5 |  _Nu" |
| 0x858E | 4 | Nu n |
| 0x859A | 4 | Nu n |
| 0x85A4 | 6 | B`Nu n |
| 0x85B0 | 7 | B Nu p8 |
| 0x85E2 | 5 |  _Nu" |
| 0x8606 | 4 | Nu n |
| 0x860B | 4 | p p8 |
| 0x8648 | 6 | "_ WBh |
| 0x866E | 4 | Nu n |
| 0x8673 | 4 | p p8 |
| 0x86B4 | 5 |  _"_B |
| 0x86C4 | 4 | $P4( |
| 0x86F2 | 4 | "@ < |
| 0x8750 | 5 | `$  k |
| 0x8806 | 6 | RETURN |
| 0x8866 | 5 | "CNuB |
| 0x88FD | 4 | h n( |
| 0x894E | 5 | .GFAg |
| 0x8956 | 5 | .LSTf |
| 0x8962 | 13 | J@NuSave File |
| 0x8970 | 4 | Save |
| 0x8AFC | 17 | GFABASIC:GFABASIC |
| 0x8B0E | 9 |         C |
| 0x8C10 | 7 | /<BP0.! |
| 0x8C26 | 11 | NuLoad File |
| 0x8C32 | 4 | Load |
| 0x8CDA | 4 | bJ"n |
| 0x8CE4 | 4 | g "n |
| 0x8D1B | 4 | >Nup |
| 0x8D62 | 4 | NuS@ |
| 0x8D7F | 10 | RSave List |
| 0x8D8A | 6 | Save,A |
| 0x8DB9 | 7 | d  PRT: |
| 0x8DDD | 4 | D-@i |
| 0x8DE7 | 6 | Ta,".i |
| 0x8E14 | 4 | Nu=\| |
| 0x8E6B | 4 | \|N0e |
| 0x8E71 | 4 | \|N9b |
| 0x8E85 | 5 | \|LLg> |
| 0x8E8B | 5 | \|PLg@ |
| 0x8E91 | 4 | \|HEg |
| 0x8E97 | 4 | \|FOg |
| 0x8E9D | 4 | \|INg |
| 0x8EA3 | 4 | \|FFg |
| 0x8EA9 | 6 | \|SPg*` |
| 0x8F7E | 5 | SGSF? |
| 0x8F85 | 4 | F=F/ |
| 0x8F94 | 6 | HQad"_ |
| 0x8F9C | 5 | `BHQN |
| 0x8FA6 | 4 | "_*n |
| 0x8FBE | 4 | f 4< |
| 0x8FE4 | 5 | HQJ./ |
| 0x8FEA | 4 | g J. |
| 0x9002 | 4 | B B  |
| 0x900A | 5 | NuHPp |
| 0x905D | 4 | X0.! |
| 0x9070 | 12 | NuMerge File |
| 0x907D | 5 | Merge |
| 0x908A | 4 | /=/. |
| 0x909F | 4 | XJ@f |
| 0x90D5 | 4 | h"Ka |
| 0x910D | 4 | hr p |
| 0x91EF | 4 | HS@0 |
| 0x9214 | 5 | (_Nu  |
| 0x9231 | 4 | da6N |
| 0x9292 | 5 | (_NuP |
| 0x92C3 | 12 | d(_Nu <.GFA` |
| 0x92D0 | 7 |  <.LSTA |
| 0x931F | 5 | hNu0) |
| 0x934F | 7 | vgF @aD |
| 0x9367 | 6 | zg. @E |
| 0x9389 | 4 | b".i |
| 0x93B8 | 4 | Nu0. |
| 0x93C3 | 4 | dJ@g |
| 0x93CE | 4 | B@H@ |
| 0x93F7 | 4 | Dgta |
| 0x942B | 4 | ANuS |
| 0x9430 | 4 | NuJn |
| 0x9438 | 4 | Nu n |
| 0x9441 | 4 | h$I  |
| 0x9496 | 6 |  <0000 |
| 0x94A9 | 5 | h$H0. |
| 0x94D2 | 4 | gB . |
| 0x94F0 | 6 | "H _2. |
| 0x94F8 | 5 | t J@o |
| 0x9506 | 5 | S@SAf |
| 0x9513 | 6 |  "_0./ |
| 0x9547 | 4 | Pa L |
| 0x954E | 6 |  Gr 0. |
| 0x959A | 4 | R@=@ |
| 0x95B2 | 4 | Nu . |
| 0x95B7 | 5 | zg40. |
| 0x9606 | 4 | SAk4 |
| 0x961B | 10 | @f TWSWSAk |
| 0x965D | 5 | hg"S@ |
| 0x9674 | 7 |  <.DMPa |
| 0x9681 | 4 | 0J_g |
| 0x968A | 5 | al`>p |
| 0x9698 | 5 | a^`0p |
| 0x9720 | 5 | g:HSH |
| 0x975C | 5 | Nu$=a |
| 0x9783 | 4 | hp$a |
| 0x978C | 5 |  Y"Xp |
| 0x97A4 | 6 | p<v;t> |
| 0x97BF | 8 | hT@Nup%a |
| 0x9821 | 4 | Xp(a |
| 0x982C | 5 |  Q"X2 |
| 0x984D | 4 | ""Ha |
| 0x985E | 5 | @@p,a |
| 0x9874 | 5 | Nup a |
| 0x98D6 | 5 | Nu .- |
| 0x98E3 | 4 | JBn/ |
| 0x98E8 | 5 | "_ n- |
| 0x9926 | 6 | Inline |
| 0x9946 | 9 | g2 <.INLa |
| 0x9971 | 4 | z0.! |
| 0x99D5 | 6 | 6 n. 0 |
| 0x99FE | 5 | ".$ra |
| 0x9A0E | 5 | ".$va |
| 0x9A23 | 4 | 6apJ |
| 0x9A30 | 5 |  n. 0 |
| 0x9A36 | 5 | Nq*L` |
| 0x9A52 | 4 | Nuk. |
| 0x9A5C | 5 | f&R./ |
| 0x9A84 | 7 | NuB B A |
| 0x9AA8 | 4 | Dz"L |
| 0x9ACA | 5 | ".$zf |
| 0x9AD0 | 7 | >.$~=G! |
| 0x9B02 | 7 | g"=G$~B |
| 0x9B12 | 5 |  n. 0 |
| 0x9B18 | 5 | NqNua |
| 0x9B20 | 5 | -C$zf |
| 0x9B2C | 5 |  n. 0 |
| 0x9B46 | 5 | $zB./ |
| 0x9B58 | 17 | Nu-n/V/R-n/j/fNuB |
| 0x9B72 | 23 | Nu-n/N/J-n/b/^Nua&-C$vH |
| 0x9B91 | 6 | `/fNua |
| 0x9B98 | 5 | -C$rH |
| 0x9BA5 | 6 | p/ZNua |
| 0x9BEE | 7 | NuNuNuA |
| 0x9C85 | 5 | <ab-n |
| 0x9DB4 | 9 | a.a,a*a(p |
| 0x9E14 | 5 | SPNu0 |
| 0x9E44 | 5 | RPNu0 |
| 0x9E81 | 4 | PNu0 |
| 0x9EC3 | 4 | PNu0 |
| 0x9F20 | 4 | HABA |
| 0x9F26 | 5 | NuDBa |
| 0x9F2C | 5 | DBNux |
| 0x9F3A | 6 | e*RCFC |
| 0x9F61 | 7 | QRBNu&< |
| 0x9FB5 | 4 |  H@" |
| 0x9FCC | 5 | nxNu0 |
| 0x9FDF | 4 | ^ W  |
| 0x9FF3 | 4 | J$_% |
| 0xA00B | 4 | 2 _  |
| 0xA020 | 4 | $W%@ |
| 0xA04E | 6 |  Lp\|2< |
| 0xA066 | 6 | (HNu"n |
| 0xA070 | 6 |  Q !k: |
| 0xA07E | 7 | n (]Nup |
| 0xA098 | 5 | (]NuY |
| 0xA0BA | 8 | (]Nu _$n |
| 0xA0EF | 4 | \Nua |
| 0xA0F9 | 4 | RNu0 |
| 0xA143 | 4 | \|HRa |
| 0xA14A | 4 | $W5@ |
| 0xA15A | 5 | 4"J@k |
| 0xA16E | 4 | Nu"n |
| 0xA176 | 6 |  Q0!k: |
| 0xA17D | 4 | Pi"0 |
| 0xA183 | 8 | an (]Nup |
| 0xA19E | 7 | (]NuUI0 |
| 0xA1C0 | 5 | (]Nua |
| 0xA1D5 | 4 | vNu0 |
| 0xA24C | 4 | Nu"n |
| 0xA261 | 8 | !b"(]Nup |
| 0xA27C | 5 | (]NuS |
| 0xA2A0 | 4 | (]Nu |
| 0xA2AE | 8 | (]Nuav j |
| 0xA2DD | 4 | rJEj |
| 0xA2EE | 4 | a: j |
| 0xA319 | 4 | 6JBj |
| 0xA33B | 4 | < W  |
| 0xA349 | 4 | vHRa |
| 0xA34F | 4 | ($_5 |
| 0xA358 | 4 | Nu"n |
| 0xA372 | 9 | :!8!&!JBk |
| 0xA38C | 6 | (]Nu-I |
| 0xA394 | 5 | NuJEj |
| 0xA3BE | 9 | :!8!&!JEk |
| 0xA3D8 | 6 | (]Nu-I |
| 0xA3E0 | 5 | NuJBj |
| 0xA403 | 4 | t W  |
| 0xA456 | 7 | :!8!&!N |
| 0xA476 | 5 | (_NuX |
| 0xA494 | 5 | (_Nu/ |
| 0xA4E2 | 7 | (HNu Q` |
| 0xA54E | 4 | Nu$n |
| 0xA553 | 4 | \|$r  |
| 0xA55C | 4 | Nu$n |
| 0xA561 | 4 | d$r  |
| 0xA568 | 6 | Nup4*] |
| 0xA576 | 4 | (MNu |
| 0xA594 | 5 | Nu ./ |
| 0xA5B6 | 5 | (_Nu/ |
| 0xA5EA | 4 | *@(M |
| 0xA604 | 7 | Nu(]Nu0 |
| 0xA64F | 4 | (DB` |
| 0xA6D1 | 4 | l _& |
| 0xA941 | 4 | @Nua |
| 0xA968 | 5 | H@Nua |
| 0xA98C | 4 | g^ < |
| 0xA99C | 5 | k$SWg |
| 0xA9B7 | 4 | 4a>L |
| 0xA9C1 | 4 | :RWg |
| 0xA9FC | 4 | e*g, |
| 0xAA04 | 7 | b"g2JBj |
| 0xAA11 | 4 |  RBN |
| 0xAA2A | 4 | Nu4< |
| 0xAA38 | 4 | Nu4< |
| 0xAA58 | 4 | NuHz |
| 0xAA62 | 5 | "_HPN |
| 0xAA92 | 4 | NuHz |
| 0xAA9C | 5 | "_HPN |
| 0xAACC | 4 | NuHz |
| 0xAAD6 | 5 | "_HPN |
| 0xAB04 | 4 | NuHz |
| 0xAB0E | 5 | "_HPN |
| 0xAB60 | 7 | o8"_ _" |
| 0xABD9 | 4 | d"_` |
| 0xABED | 6 | P"_<.! |
| 0xABF4 | 5 | JFk"` |
| 0xAC26 | 5 | &_NuJ |
| 0xAC2E | 5 | 0./<g |
| 0xAC5B | 4 | F~dt |
| 0xACE6 | 34 | NuCON:10/10/320/120/New CLI Window |
| 0xAD16 | 4 | Nu < |
| 0xAD24 | 4 | Nu < |
| 0xAD32 | 16 | Nuprinter.device |
| 0xAD4B | 4 | Fg"N |
| 0xAD8A | 4 |  @ ( |
| 0xAD8F | 4 | .$@L |
| 0xADB8 | 7 | gb*@p>N |
| 0xADC2 | 7 | UP&@"@A |
| 0xADFF | 4 | z"KN |
| 0xAE07 | 6 | h"Kp>N |
| 0xAE11 | 4 | ~"MN |
| 0xAE20 | 4 | NuHz |
| 0xAE7C | 5 | kz&Y2 |
| 0xAEC0 | 5 | AP"n- |
| 0xAEE1 | 4 | @"Kp |
| 0xAF79 | 4 | F"0p |
| 0xAFC1 | 5 | B$_R@ |
| 0xAFF6 | 5 | Nu<.! |
| 0xB003 | 4 | F"0` |
| 0xB039 | 4 | \-@- |
| 0xB04E | 4 | "@ < |
| 0xB05B | 5 | ~Nu < |
| 0xB0A8 | 6 | g\|"@ n |
| 0xB125 | 4 | 4p=N |
| 0xB152 | 5 | &@"@A |
| 0xB199 | 6 | nBXBX  |
| 0xB1CF | 4 | C$r0 |
| 0xB226 | 5 | "KpJN |
| 0xB22F | 4 | ~"EN |
| 0xB23E | 17 | Nunarrator.device |
| 0xB290 | 4 | NuHz |
| 0xB2B3 | 4 | 0$HJ |
| 0xB33D | 4 | 2NuN |
| 0xB3FE | 7 | R@NuNuN |
| 0xB481 | 4 | hHP  |
| 0xB4AC | 4 | D@3@ |
| 0xB5E0 | 6 | UU""UU |
| 0xB61E | 4 |   @@ |
| 0xB624 | 4 | AA"" |
| 0xB644 | 6 | PP     |
| 0xB688 | 4 | 66cc |
| 0xB696 | 4 | ""AA |
| 0xB6C2 | 4 | AA>> |
| 0xB6D0 | 4 | BB$$ |
| 0xB700 | 4 | ""DD |
| 0xB708 | 4 | DD"" |
| 0xB718 | 4 |   @@ |
| 0xB72C | 4 | 00BB |
| 0xB734 | 4 | BB$$ |
| 0xB8BF | 4 | FgJN |
| 0xB8D3 | 4 | Fg6R |
| 0xB8E1 | 4 | Fg(N |
| 0xB90C | 6 | NuNuHy |
| 0xB95A | 5 | ("$HH |
| 0xB9AE | 5 | DBJCj |
| 0xB9B8 | 5 | a\|&_H |
| 0xB9C4 | 5 | )FJ.i |
| 0xB9E2 | 5 | $d"KN |
| 0xB9EA | 5 | $tJ.i |
| 0xBA78 | 4 | &G.+ |
| 0xBA7D | 5 | 2&G k |
| 0xBABF | 4 | \-@- |
| 0xBADE | 5 | NupJN |
| 0xBAF4 | 5 | Nu..( |
| 0xBB11 | 4 | 2 GB |
| 0xBB18 | 5 | Nu..( |
| 0xBB2E | 5 | g0&GG |
| 0xBB38 | 4 | "B&i |
| 0xBB6A | 5 | "G .- |
| 0xBBBB | 4 | !NuN |
| 0xBBED | 4 | Gg$S |
| 0xBC7C | 4 |  G.( |
| 0xBC81 | 4 | . GA |
| 0xBC92 | 4 | NuHz |
| 0xBCEA | 5 | ($ _< |
| 0xBCF6 | 5 | (" _< |
| 0xBD02 | 5 | S@"_A |
| 0xBD08 | 4 | ("#X |
| 0xBD0D | 4 | $Nuz |
| 0xBD9A | 5 | 2 0 6 |
| 0xBDA6 | 5 | $H&IN |
| 0xBDE5 | 4 | H"_N |
| 0xBE3A | 5 | Nu n' |
| 0xBE5B | 6 | 2Nu GC |
| 0xBE63 | 4 | TNua |
| 0xBE72 | 7 | g$$H..' |
| 0xBEAC | 5 | f"".( |
| 0xBEE5 | 4 |  DA2 |
| 0xBF13 | 6 | Df`,.' |
| 0xBF42 | 5 | g0 @z |
| 0xBF54 | 7 | DBDC$HJ |
| 0xBF66 | 5 |  F"Jp |
| 0xBF74 | 5 | Nu,.' |
| 0xBF84 | 4 | "H G |
| 0xBF9E | 5 | "FpPN |
| 0xC01B | 5 | B\|C ! |
| 0xC02C | 5 | $H"$q |
| 0xC16C | 4 | ("#X |
| 0xC171 | 4 | $p N |
| 0xC18F | 5 | INuHy |
| 0xC1BD | 6 | E=Gi < |
| 0xC1C5 | 6 | D=Fi"< |
| 0xC1CD | 6 | E=Fi$< |
| 0xC1D5 | 4 | D=Fi |
| 0xC1E2 | 5 | >.i 4 |
| 0xC202 | 7 | 2.i$aFL |
| 0xC210 | 5 | 0.i"2 |
| 0xC218 | 7 | >.i$a0L |
| 0xC226 | 5 | <.i"6 |
| 0xC232 | 5 | 2.i a |
| 0xC244 | 4 | ("#X |
| 0xC249 | 6 | $pDNua |
| 0xC323 | 4 | \ PN |
| 0xC34E | 5 | Nu"X0 |
| 0xC380 | 4 | Nu$n |
| 0xC3AC | 8 | (CNup\B. |
| 0xC422 | 5 | c*2 n |
| 0xC45C | 4 | (_B. |
| 0xC462 | 4 | Nu&n |
| 0xC491 | 4 | C2;0 |
| 0xC592 | 5 | NupZN |
| 0xC5DE | 4 | Nu&n |
| 0xC5E4 | 7 | 0#f  #o |
| 0xC5F0 | 4 | (@Nu |
| 0xC626 | 5 | (@NuP |
| 0xC679 | 4 | B 0  |
| 0xC6B3 | 4 | C 00 |
| 0xC6D4 | 5 | g QAe |
| 0xC700 | 4 | Nu$n |
| 0xC744 | 5 | (HNup |
| 0xC753 | 4 | B @0 |
| 0xC760 | 4 | (_Nu |
| 0xC840 | 4 | kZ"n |
| 0xC892 | 5 | Nup*N |
| 0xC9A2 | 4 | l6Bk |
| 0xC9A8 | 4 |  K0* |
| 0xCA2B | 4 | n-H/ |
| 0xCA3D | 4 | X-H/ |
| 0xCB0F | 6 | &"_ _& |
| 0xCBAE | 6 |  @HP$n |
| 0xCBD2 | 4 |  _"H |
| 0xCBFE | 5 | HU*HN |
| 0xCC12 | 4 | *_Nu |
| 0xCC25 | 4 | pHQa |
| 0xCC2B | 4 | :"_R |
| 0xCCBA | 5 | 0 "`k |
| 0xCCFA | 6 | d.$@0* |
| 0xCD3A | 4 | UJ&" |
| 0xCD50 | 9 | /"/"/"/"Q |
| 0xCD6C | 5 | NupGN |
| 0xCD8D | 4 | $&@L |
| 0xCDAF | 4 | G.0p |
| 0xCDE6 | 5 | "@$n' |
| 0xCE32 | 4 | "F<) |
| 0xCE64 | 5 | Xp$_$ |
| 0xCE90 | 7 |  _"P$@  |
| 0xCED5 | 4 | 2NuG |
| 0xCF39 | 4 | A`$* |
| 0xCF6D | 4 | h$HH |
| 0xCFB6 | 5 | g "n' |
| 0xCFCF | 4 | L n' |
| 0xCFD7 | 4 | 2NuH |
| 0xD080 | 7 |  @Nu @p |
| 0xD0D8 | 4 | ]PNu |
| 0xD102 | 5 |  WaNa |
| 0xD10E | 6 |  _`x/\| |
| 0xD151 | 8 | l`6HP KN |
| 0xD16A | 4 | "@() |
| 0xD186 | 9 |  _NuHP JN |
| 0xD193 | 4 | 6 _  |
| 0xD1A2 | 5 | "n.&" |
| 0xD1B6 | 5 | "n.&, |
| 0xD1C9 | 4 | QNqg |
| 0xD1CE | 9 | ,<NuNq4<` |
| 0xD1DA | 9 | ,<NqNq4<a |
| 0xD1FA | 5 | U"-@- |
| 0xD259 | 4 | RHPN |
| 0xD287 | 4 | B _` |
| 0xD2AD | 6 | ?NupcN |
| 0xD2C4 | 4 | $W * |
| 0xD2C9 | 4 |   @$ |
| 0xD2F9 | 4 | h$H$ |
| 0xD312 | 5 | "_ _N |
| 0xD345 | 4 | */.' |
| 0xD362 | 5 | $n&Dr |
| 0xD370 | 5 | RA&ZG |
| 0xD410 | 8 |  KatHP"h |
| 0xD423 | 5 | V$H"j |
| 0xD435 | 4 | ~ _N |
| 0xD445 | 4 | 6 .' |
| 0xD451 | 4 | \|NuN |
| 0xD47E | 5 | Nu .' |
| 0xD486 | 6 |  @HP"h |
| 0xD48D | 5 | \|HQ/) |
| 0xD493 | 7 | ~g.$I i |
| 0xD4BE | 7 | .~ _NuP |
| 0xD516 | 5 | &@"@B |
| 0xD54C | 4 | "J i |
| 0xD558 | 5 |  J"KN |
| 0xD586 | 5 | f6..' |
| 0xD5A9 | 4 | C G` |
| 0xD5D4 | 5 | ,` JN |
| 0xD60F | 4 | @gLG |
| 0xD684 | 5 | Nu @p |
| 0xD6B2 | 7 |  @Nu @p |
| 0xD6E2 | 4 |  DHz |
| 0xD760 | 5 | g6 A. |
| 0xD79E | 6 | g@ J&( |
| 0xD7D6 | 5 | "CpPN |
| 0xD7DF | 4 | ~NuN |
| 0xD7F8 | 4 | HP&( |
| 0xD808 | 5 | $_&_2 |
| 0xD834 | 4 |  @SB |
| 0xD88E | 5 | rOSAk |
| 0xD8D7 | 5 | \|401@ |
| 0xD8E9 | 6 | 32;44m |
| 0xD8F0 | 5 | Nu".' |
| 0xD90C | 5 |  ANup |
| 0xD912 | 5 |  ANuJ |
| 0xD94C | 4 |  C"h |
| 0xD961 | 4 | ~"C  |
| 0xD9BF | 4 | 7gJN |
| 0xD9E8 | 4 |  A0( |
| 0xDA20 | 4 | gTr" |
| 0xDA2A | 5 | gJ @  |
| 0xDA56 | 4 | $.&@ |
| 0xDA5C | 5 | -B&@L |
| 0xDA6B | 4 | ~"K  |
| 0xDA75 | 4 | ~p>N |
| 0xDAF0 | 4 | "E / |
| 0xDB16 | 4 | SG+J |
| 0xDB40 | 6 | &C$D'j |
| 0xDB52 | 4 | aP"* |
| 0xDB5E | 5 | aX"2H |
| 0xDB64 | 7 | fh"2H g |
| 0xDB6E | 7 |  Aa2aDJ |
| 0xDBCB | 4 | "NuA |
| 0xDBD7 | 5 | !f$RG |
| 0xDBE3 | 4 | p"rH |
| 0xDC39 | 4 | & n' |
| 0xDC47 | 6 | :TD*Kt |
| 0xDC7A | 4 |  X;B |
| 0xDC9E | 4 | pB"( |
| 0xDCA4 | 4 | g2"A |
| 0xDCE3 | 5 | @*F m |
| 0xDCF3 | 4 | D P  |
| 0xDD04 | 5 | aD"Q  |
| 0xDD38 | 4 | $.&@ |
| 0xDD3E | 7 | -B&@"KN |
| 0xDD4D | 6 | p$@&@v |
| 0xDD70 | 4 | X@!I |
| 0xDDB1 | 4 | D$R  |
| 0xDDE1 | 4 | G"0p |
| 0xDE12 | 5 | Nu$H& |
| 0xDE76 | 4 | HQ ) |
| 0xDE8C | 6 | f:`>0) |
| 0xDE98 | 5 | g@ BN |
| 0xDEA0 | 6 |  \|".&@ |
| 0xDED4 | 9 | HRaJ$_"_N |
| 0xDEE1 | 4 | \|NuN |
| 0xDF20 | 4 | Nu$n |
| 0xDF42 | 5 | (ANuA |
| 0xDF55 | 4 | hNuN |
| 0xDF6C | 4 | HQ$j |
| 0xE028 | 5 | HT(GI |
| 0xE03A | 5 | "_SFJ |
| 0xE040 | 5 | f$$Y" |
| 0xE05C | 5 | aD"_Q |
| 0xE064 | 9 | `0JFk,$Y" |
| 0xE09E | 4 | (_Nu |
| 0xE115 | 4 | \ p  |
| 0xE129 | 6 | APADAH |
| 0xE18D | 4 | l-Mi |
| 0xE19A | 4 | "@ ) |
| 0xE1A2 | 5 | *@ @N |
| 0xE1B3 | 6 | ARBRC2 |
| 0xE234 | 5 | SDSE6 |
| 0xE23C | 7 | RDRE FC |
| 0xE266 | 7 | SC R"K  |
| 0xE30E | 4 | `""o |
| 0xE318 | 5 | RFRGE |
| 0xE351 | 4 | @ J  |
| 0xE3A2 | 5 | RARBN |
| 0xE3D0 | 5 | RDREH |
| 0xE444 | 5 | bhJAg |
| 0xE474 | 5 | g,&@E |
| 0xE4AD | 4 | ~pYN |
| 0xE4B2 | 5 |  F$X0 |
| 0xE4DA | 4 |  D(( |
| 0xE4DF | 4 | . DA |
| 0xE51C | 5 | +L"Fp |
| 0xE58A | 4 | Nu 9 |
| 0xE618 | 5 | Nu:.g |
| 0xE626 | 4 | hFNu |
| 0xE63B | 4 | <\IQ |
| 0xE65B | 4 | <Nua |
| 0xE668 | 4 | g$Hi |
| 0xE6A5 | 4 | v$YB |
| 0xE6D3 | 5 | H$Y%\| |
| 0xE6FA | 4 | Nu%j |
| 0xE750 | 5 | NuRng |
| 0xE764 | 5 | hDJZg |
| 0xE76E | 5 | UJNuN |
| 0xE792 | 5 | $YNuH |
| 0xE7BF | 4 | \|"@C |
| 0xE83C | 5 | NuJ.g |
| 0xE868 | 5 | Nu:.g |
| 0xE876 | 7 | hF$Ya<` |
| 0xE88C | 5 | g*a(a |
| 0xE8D8 | 4 | HQJ) |
| 0xE8DE | 5 | n("nh |
| 0xE8EE | 4 | +:"j |
| 0xE928 | 7 | 0~"W]IG |
| 0xE930 | 4 | hD(< |
| 0xE952 | 7 | BS"_NuH |
| 0xE980 | 4 |  @"h |
| 0xE994 | 4 |  @"h |
| 0xE999 | 4 | T-Hh |
| 0xE9D3 | 6 | 7flRMa |
| 0xE9F2 | 4 | R.h> |
| 0xEA0D | 4 | :pT6 |
| 0xEA3F | 6 | `NuHRN |
| 0xEA5B | 4 | pJ.g |
| 0xEA60 | 4 | f""k |
| 0xEA6B | 7 | HS@SABh |
| 0xEA84 | 5 | &_$_a |
| 0xEAC2 | 4 | XA1A |
| 0xEACB | 5 |  T@1@ |
| 0xEAEA | 4 |  D!@ |
| 0xEAF4 | 8 | -@h6-Eh: |
| 0xEAFD | 6 | MSE"@2 |
| 0xEB0D | 4 | BaRA |
| 0xEB20 | 5 | gp DA |
| 0xEB44 | 4 |  @1j |
| 0xEB4B | 7 | :-@h.-\| |
| 0xEB56 | 5 | h2BZ4 |
| 0xEB8F | 12 |  Nu"nh6 .h:a |
| 0xEB9E | 9 | "nh. .h2a |
| 0xEBF0 | 5 | NupTN |
| 0xEC38 | 5 | NupcN |
| 0xEC56 | 4 | Nu,y |
| 0xECE1 | 4 | 9Nua |
| 0xED13 | 4 | "NuN |
| 0xED38 | 4 | hD0( |
| 0xED74 | 5 | hF$YJ |
| 0xED81 | 4 | <\IQ |
| 0xED98 | 4 | hF:9 |
| 0xEDC0 | 7 | "X\H$PE |
| 0xEDED | 4 | H .g |
| 0xEDF4 | 4 | "@ < |
| 0xEE01 | 4 | ~Nup |
| 0xEE2D | 4 | J02` |
| 0xEE43 | 50 | "%(+.1369<?ADGILNQSUXZ\^`bdfhjkmopqstuwxyzz{\|}}~~~ |
| 0xEE78 | 12 | audio.device |
| 0xEEA2 | 4 | "C&) |
| 0xEED0 | 5 | "ni&N |
| 0xEEE4 | 5 | "FpDN |
| 0xEEF1 | 4 | $HRN |
| 0xEF97 | 4 | 8.00 |
| 0xEFA6 | 4 |  G.( |
| 0xEFDA | 4 | NuJ9 |
| 0xEFF6 | 4 | NuJ9 |
| 0xF01C | 6 | NuHP$@ |
| 0xF03B | 5 | \ _%@ |
| 0xF049 | 5 |   @ < |
| 0xF060 | 4 | NuHx |
| 0xF12A | 5 | iZ,00 |
| 0xF235 | 4 | B p  |
| 0xF23D | 5 |  gfHP |
| 0xF30E | 4 | i&&p |
| 0xF348 | 5 | NupAN |
| 0xF3D1 | 8 | @..i&fnH |
| 0xF3F3 | 4 | ,pDN |
| 0xF40E | 5 | "ni&B |
| 0xF42A | 5 | i&"Zt |
| 0xF49A | 7 | ,GNu"KN |
| 0xF4E6 | 4 |  n.* |
| 0xF4ED | 6 | ` n.&0 |
| 0xF505 | 4 | ` n( |
| 0xF52A | 5 | "@&n- |
| 0xF53A | 4 |  @!K |
| 0xF566 | 4 |  @"n |
| 0xF56B | 4 | P$Ip |
| 0xF572 | 5 | cP$IL |
| 0xF594 | 5 | z,\|(( |
| 0xF5C0 | 7 | "_$IJ./ |
| 0xF61E | 5 | g&Bn/ |
| 0xF631 | 5 | AS@=@ |
| 0xF65D | 4 | hr " |
| 0xF680 | 4 | e =\| |
| 0xF691 | 4 | VJ.i |
| 0xF6AF | 6 | ,`6"@N |
| 0xF6B9 | 4 | \| n- |
| 0xF6FB | 4 | H2.. |
| 0xF701 | 4 | IUAk |
| 0xF725 | 6 | TS@kDL |
| 0xF8F1 | 5 | Pgt0. |
| 0xF905 | 4 | ""./ |
| 0xF929 | 4 | T-A/ |
| 0xF961 | 5 | .b*=@ |
| 0xFAB1 | 4 | ."ng |
| 0xFD03 | 4 | 'a\J |
| 0xFD6C | 4 | a"-n |
| 0xFDC3 | 4 | ~$_B |
| 0xFDE5 | 5 | 0Bg n |
| 0xFDEF | 5 | 'k~>( |
| 0xFF0F | 8 | HS@"HBYQ |
| 0xFF2D | 4 | HS@0 |
| 0xFFC5 | 5 | 'fJ n |
| 0x10029 | 4 | &BXP |
| 0x1003C | 4 | NuBn |
| 0x10096 | 5 |  n.&/ |
| 0x1009E | 5 | Nu n( |
| 0x100B2 | 4 | R40( |
| 0x10104 | 7 | S, n.&  |
| 0x10172 | 4 | $I=F |
| 0x10193 | 4 | T ng |
| 0x101D0 | 4 | $I-J |
| 0x101ED | 4 | T ng |
| 0x1023F | 4 | \`.a |
| 0x1027C | 4 |  n.* |
| 0x1028A | 4 |  n.* |
| 0x102B4 | 4 |  n.* |
| 0x102F3 | 5 | %Nu0. |
| 0x10305 | 4 | "*./ |
| 0x10311 | 4 | T$ng |
| 0x10324 | 5 | `BRE& |
| 0x10364 | 5 | YATAQ |
| 0x103A4 | 4 | B B  |
| 0x103B8 | 4 | g*k( |
| 0x103C2 | 4 | r(B0 |
| 0x10405 | 6 | T I$ng |
| 0x10432 | 4 | gH<" |
| 0x10471 | 5 | "NuTB |
| 0x104AB | 5 | "NuTB |
| 0x104BD | 4 | " ./ |
| 0x1051E | 6 | YBTB=B |
| 0x10525 | 4 | "NuP |
| 0x10537 | 7 | Hd.W@kn |
| 0x1055F | 6 | PH@B@` |
| 0x10585 | 5 | * n.* |
| 0x1058D | 4 | rB./ |
| 0x10625 | 7 | h J"Wv< |
| 0x10643 | 4 | h"WJ |
| 0x10652 | 5 | SI$_4 |
| 0x10686 | 4 | aJ`  |
| 0x106C9 | 4 | 4g^0 |
| 0x106EC | 5 | B B L |
| 0x10707 | 6 | hSA JG |
| 0x1072A | 6 | NuFind |
| 0x10731 | 7 | Replace |
| 0x10802 | 60 |    \| Copy \| Move \|Write \|Llist \|Start \| End  \| ^Del \| Hide \| |
| 0x1083F | 9 | Block ??? |
| 0x10849 | 15 | Cursor in block |
| 0x108A9 | 6 | "f40.. |
| 0x108B1 | 6 | HW@2.. |
| 0x108B9 | 4 | If"H |
| 0x108D6 | 6 | CMWLSE |
| 0x10966 | 4 | @d"n |
| 0x10994 | 4 | `& . |
| 0x10A67 | 7 |  zO$H n |
| 0x10A78 | 5 | &nH\I |
| 0x10AF9 | 5 | /Nu n |
| 0x10B31 | 4 | 0NuP |
| 0x10B52 | 4 | $H n |
| 0x10B5E | 5 | &nH\I |
| 0x10B8B | 47 | \| Save \|Save,A\| Quit \| New  \|BlkSta\|Replac\| Pg  |
| 0x10BBB | 23 |  \|Normal\|Direct\| Run  \| |
| 0x10BD7 | 47 | \| Load \|Merge \|Llist \|Block \|BlkEnd\| Find \| Pg  |
| 0x10C07 | 23 |  \|Insert\|ClkOn \| Test \| |
| 0x10C20 | 7 | Overwrp |
| 0x10C4A | 4 | T@"n |
| 0x10C79 | 5 | K[C=C |
| 0x10C8A | 6 | Nu n.& |
| 0x10CA0 | 6 | n  n.& |
| 0x10CB0 | 7 | ff n.`p |
| 0x10CD4 | 4 | H\ h |
| 0x10DE4 | 4 | 0000 |
| 0x10DEA | 5 | 0000B |
| 0x10E18 | 5 |  JpJr |
| 0x10E49 | 4 | T ng |
| 0x10E97 | 4 | XNut |
| 0x10EA4 | 5 |  JRBJ |
| 0x10F22 | 4 | gZJ. |
| 0x10F33 | 5 | 9b,J. |
| 0x10F65 | 4 | bSAk |
| 0x10F9A | 4 | NuH@ |
| 0x11025 | 17 | X _NuClear Inline |
| 0x11037 | 11 | Clear\|Error |
| 0x11044 | 38 |  Inline   \| Load \| Save \| Dump \|Clear  |
| 0x1108F | 4 | If H |
| 0x110AC | 5 | xLSDC |
| 0x1113B | 4 | h"H0 |
| 0x112E6 | 213 | GFA-BASIC 3.52+L  Load+S  Save+N  New Names     +R  Run+0  Taskpri 0+1  Taskpri 1+C  Cleanup  NewCLI  Save Icon!  Deletable!  Not Deletable!+I  Save Icon Block+U Cut+O Copy+P Paste+B Blk Start     +K Blk EndNewCLI |
| 0x113BC | 4 | NIL: |
| 0x11466 | 11 | RAM:GFA.$$$ |
| 0x11472 | 7 | F./8ap` |
| 0x11499 | 4 | CJ@g |
| 0x114E7 | 4 | Z n( |
| 0x114F0 | 7 | ~RJ./8g |
| 0x11504 | 4 | ~R?< |
| 0x1151F | 4 | S Wa |
| 0x11526 | 5 |  _~R\| |
| 0x11570 | 4 | Nu : |
| 0x11576 | 4 | Nu : |
| 0x1157C | 4 | Nu : |
| 0x11582 | 5 | NupKN |
| 0x115B2 | 5 | ,GNua |
| 0x115C2 | 5 | ,GNua |
| 0x115D2 | 5 | ,GNua |
| 0x115E2 | 5 | ,GNua |
| 0x115F2 | 5 | ,GNua |
| 0x11602 | 5 | ,GNua |
| 0x11612 | 5 | ,GNua |
| 0x11624 | 5 | ,GNu. |
| 0x11632 | 5 | ,GNu. |
| 0x1163F | 6 | \|,GNua |
| 0x11651 | 6 | (,GNua |
| 0x11664 | 5 | ,GNua |
| 0x11676 | 5 | ,GNu. |
| 0x11684 | 5 | ,GNua |
| 0x11695 | 6 | .,GNua |
| 0x116A8 | 5 | ,GNua |
| 0x116BA | 5 | ,GNua |
| 0x116CC | 5 | ,GNua |
| 0x116DD | 6 | L,GNua |
| 0x116EF | 6 | F,GNua |
| 0x11702 | 5 | ,GNua |
| 0x11713 | 6 | 4,GNua |
| 0x11726 | 5 | ,GNuN |
| 0x1173F | 6 | v,GNua |
| 0x11751 | 6 | @,GNua |
| 0x11763 | 6 | :,GNua |
| 0x11775 | 6 | R,GNua |
| 0x11787 | 6 | p,GNua |
| 0x11799 | 6 | ",GNua |
| 0x117AB | 6 | j,GNua |
| 0x117BD | 6 | d,GNu. |
| 0x117CC | 5 | ,GNu. |
| 0x117DA | 7 | ,GNuNu. |
| 0x117EA | 5 | ,GNuN |
| 0x117FC | 5 | ,GNuN |
| 0x11804 | 5 | "@$A  |
| 0x11814 | 5 | ,GNuN |
| 0x1181C | 7 |  @"A$B  |
| 0x11830 | 5 | ,GNua |
| 0x11842 | 5 | ,GNua |
| 0x11854 | 5 | ,GNuN |
| 0x11868 | 5 | ,GNuN |
| 0x11887 | 4 | "*@. |
| 0x118A4 | 5 | ,GNu. |
| 0x118B2 | 5 | ,GNu. |
| 0x118C0 | 5 | ,GNu. |
| 0x118CD | 6 | \|,GNu. |
| 0x118DB | 6 | v,GNuN |
| 0x118ED | 6 | p,GNu. |
| 0x118FB | 6 | j,GNuN |
| 0x1190D | 8 | d,GNua@. |
| 0x1191D | 8 | ^,GNua0. |
| 0x1192D | 8 | X,GNua . |
| 0x1193D | 6 | R,GNua |
| 0x1194F | 6 | L,GNuN |
| 0x11958 | 5 | "ANua |
| 0x11969 | 6 | F,GNua |
| 0x1197B | 6 | @,GNuN |
| 0x1198D | 6 | :,GNua |
| 0x1199D | 6 | 4,GNua |
| 0x119AF | 6 | .,GNua |
| 0x119C1 | 6 | (,GNua |
| 0x119D3 | 6 | ",GNua |
| 0x119E6 | 5 | ,GNua |
| 0x119F8 | 5 | ,GNua |
| 0x11A0A | 5 | ,GNua |
| 0x11A1C | 5 | ,GNua |
| 0x11A2E | 5 | ,GNua |
| 0x11A40 | 5 | ,GNua |
| 0x11A52 | 5 | ,GNua |
| 0x11A64 | 5 | ,GNua |
| 0x11A76 | 5 | ,GNuN |
| 0x11A7E | 7 | "@$A&B. |
| 0x11A8E | 5 | ,GNua |
| 0x11AA0 | 5 | ,GNua |
| 0x11AB2 | 5 | ,GNua |
| 0x11AC4 | 5 | ,GNuN |
| 0x11AD6 | 5 | ,GNuN |
| 0x11AE8 | 5 | ,GNuN |
| 0x11AFA | 5 | ,GNua |
| 0x11B0C | 5 | ,GNuN |
| 0x11B1E | 5 | ,GNuN |
| 0x11B30 | 5 | ,GNuN |
| 0x11B42 | 5 | ,GNuN |
| 0x11B54 | 5 | ,GNua |
| 0x11B66 | 5 | ,GNua |
| 0x11B78 | 5 | ,GNua |
| 0x11B8A | 5 | ,GNua |
| 0x11B9C | 5 | ,GNua |
| 0x11BAE | 5 | ,GNua |
| 0x11BC0 | 5 | ,GNua |
| 0x11BD1 | 6 | z,GNua |
| 0x11BE3 | 6 | t,GNua |
| 0x11BF5 | 6 | n,GNua |
| 0x11C07 | 6 | h,GNua |
| 0x11C29 | 6 | b,GNua |
| 0x11C3D | 6 | \,GNua |
| 0x11C4F | 6 | V,GNua |
| 0x11C61 | 6 | P,GNua |
| 0x11C73 | 6 | J,GNua |
| 0x11C87 | 6 | D,GNua |
| 0x11C99 | 6 | >,GNua |
| 0x11CAB | 6 | 8,GNua |
| 0x11CBD | 6 | 2,GNua |
| 0x11CCF | 6 | ,,GNua |
| 0x11CE1 | 6 | &,GNua |
| 0x11CF3 | 6 |  ,GNua |
| 0x11D06 | 5 | ,GNua |
| 0x11D18 | 5 | ,GNua |
| 0x11D2A | 5 | ,GNu. |
| 0x11D38 | 5 | ,GNu. |
| 0x11D46 | 5 | ,GNuN |
| 0x11D58 | 5 | ,GNuN |
| 0x11D60 | 9 |  @"A$B&C. |
| 0x11D72 | 5 | ,GNu. |
| 0x11D80 | 5 | ,GNua |
| 0x11D92 | 5 | ,GNua |
| 0x11DA4 | 5 | ,GNua |
| 0x11DB6 | 5 | ,GNuN |
| 0x11DF6 | 22 | ,GNutranslator.library |
| 0x11E0D | 12 | icon.library |
| 0x11E1A | 13 | potgo.library |
| 0x11E28 | 16 | diskfont.library |
| 0x11E39 | 13 | timer.library |
| 0x11E55 | 6 | 2,GNua |
| 0x11E67 | 6 | >,GNua |
| 0x11E7A | 5 | ,GNuN |
| 0x11E82 | 5 |  @"A  |
| 0x11E95 | 6 | J,GNua |
| 0x11EA7 | 6 | t,GNua |
| 0x11EB9 | 6 | n,GNuN |
| 0x11ED8 | 5 | ,GNua |
| 0x11EEA | 5 | ,GNuN |
| 0x11F08 | 5 | ,GNua |
| 0x11F1A | 5 | ,GNua |
| 0x11F2C | 5 | ,GNua |
| 0x11F3E | 5 | ,GNua |
| 0x11F50 | 5 | ,GNua |
| 0x11F62 | 5 | ,GNu. |
| 0x11F70 | 5 | ,GNua |
| 0x11F82 | 5 | ,GNuN |
| 0x11F98 | 5 | ,GNua |
| 0x11FAA | 5 | ,GNuN |
| 0x11FBC | 5 | ,GNua |
| 0x11FCE | 5 | ,GNua |
| 0x11FE0 | 5 | ,GNua |
| 0x11FF2 | 5 | ,GNua |
| 0x12004 | 5 | ,GNua |
| 0x12015 | 6 | h,GNua |
| 0x12028 | 5 | ,GNua |
| 0x1203A | 5 | ,GNua |
| 0x1204B | 6 | \|,GNuN |
| 0x12065 | 6 | V,GNua |
| 0x12077 | 6 | v,GNua |
| 0x1208A | 5 | ,GNua |
| 0x1209C | 5 | ,GNua |
| 0x120AD | 6 | p,GNuN |
| 0x120BF | 6 | b,GNua |
| 0x120D2 | 5 | ,GNua |
| 0x120E3 | 6 | j,GNuN |
| 0x12101 | 6 | d,GNua |
| 0x12113 | 6 | ^,GNua |
| 0x12125 | 6 | X,GNua |
| 0x12137 | 6 | R,GNua |
| 0x12149 | 6 | L,GNua |
| 0x1215B | 6 | F,GNua |
| 0x1216D | 6 | @,GNua |
| 0x1217F | 6 | :,GNua |
| 0x12191 | 6 | 4,GNu. |
| 0x1219F | 6 | .,GNua |
| 0x121B1 | 6 | (,GNua |
| 0x121C3 | 6 | ",GNua |
| 0x121D5 | 6 | P,GNua |
| 0x121E7 | 6 | 8,GNu. |
| 0x121F6 | 5 | ,GNua |
| 0x12208 | 5 | ,GNua |
| 0x12219 | 6 | D,GNuN |
| 0x1222E | 5 | ,GNua |
| 0x12240 | 5 | ,GNu. |
| 0x1224D | 6 | z,GNua |
| 0x12260 | 5 | ,GNua |
| 0x12272 | 5 | ,GNua |
| 0x12284 | 5 | ,GNua |
| 0x12296 | 5 | ,GNuN |
| 0x1229E | 5 |  @"A  |
| 0x122B4 | 5 | ,GNua |
| 0x122C6 | 5 | ,GNua |
| 0x122D8 | 5 | ,GNua |
| 0x122EA | 5 | ,GNua |
| 0x122FC | 5 | ,GNua |
| 0x1230D | 6 | \,GNu. |
| 0x1231C | 5 | ,GNua |
| 0x1232E | 5 | ,GNu. |
| 0x1233C | 5 | ,GNu. |
| 0x1234A | 5 | ,GNuN |
| 0x12364 | 5 | ,GNua |
| 0x12376 | 5 | ,GNua |
| 0x12388 | 5 | ,GNuN |
| 0x123BE | 5 |  @"@  |
| 0x123CE | 5 |  @"A  |
| 0x123DC | 9 |  @"A$BNuN |
| 0x123E8 | 5 |  @"A  |
| 0x1240C | 7 |  @"ANuN |
| 0x12416 | 5 |  ANuN |
| 0x12432 | 5 |  @NuN |
| 0x1243A | 5 | "@NuN |
| 0x1244C | 5 | ,GNua |
| 0x1245C | 5 | ,GNua |
| 0x1246C | 5 | ,GNua |
| 0x12484 | 5 | ,GNua |
| 0x12495 | 6 | F,GNua |
| 0x124A6 | 5 | ,GNua |
| 0x124B8 | 6 | ,GNu2< |
| 0x124D0 | 4 | *@,z |
| 0x12504 | 7 | ,_Nuah. |
| 0x12514 | 5 | ,GNua |
| 0x12526 | 5 | ,GNuN |
| 0x1254E | 5 | ,GNuN |
| 0x12556 | 5 | "@ A  |
| 0x1256E | 5 | ,GNuN |
| 0x125A6 | 5 | ,GNua |
| 0x125B8 | 5 | ,GNua |
| 0x125CA | 5 | ,GNua |
| 0x125DA | 5 | ,GNua |
| 0x125EB | 6 | >,GNu. |
| 0x125F9 | 6 | 2,GNua |
| 0x1260C | 5 | ,GNua |
| 0x1261E | 5 | ,GNua |
| 0x12637 | 6 | L,GNuN |
| 0x12652 | 5 | ,GNua |
| 0x12664 | 5 | ,GNua |
| 0x12676 | 5 | ,GNua |
| 0x12688 | 5 | ,GNua |
| 0x1269A | 5 | ,GNua |
| 0x126AC | 5 | ,GNuN |
| 0x126BE | 5 | ,GNua |
| 0x126D0 | 5 | ,GNua |
| 0x126E2 | 5 | ,GNuN |
| 0x126FB | 6 | z,GNua |
| 0x1270D | 6 | :,GNua |
| 0x1271F | 6 | ,,GNua |
| 0x12732 | 5 | ,GNua |
| 0x12743 | 6 | 4,GNua |
| 0x12755 | 6 | @,GNua |
| 0x12767 | 7 | ",GNu?< |
| 0x12789 | 6 | (,GNua |
| 0x1279C | 5 | ,GNua |
| 0x127AD | 6 | .,GNuN |
| 0x127C0 | 5 | ,GNua |
| 0x127D2 | 5 | ,GNua |
| 0x127E4 | 5 | ,GNu. |
| 0x127F1 | 6 | 8,GNuN |
| 0x12806 | 5 | ,GNuN |
| 0x1282A | 5 | ,GNua |
| 0x1283C | 5 | ,GNua |
| 0x1284E | 5 | ,GNua |
| 0x12860 | 5 | ,GNuN |
| 0x12879 | 6 | t,GNua |
| 0x1288C | 5 | ,GNua |
| 0x128AC | 5 | ,GNua |
| 0x128BE | 5 | ,GNua |
| 0x128D0 | 5 | ,GNua |
| 0x128E7 | 4 |  Nua |
| 0x128F7 | 4 |  Nua |
| 0x12908 | 5 | ,GNua |
| 0x1291C | 5 | ,GNua |
| 0x12930 | 5 | ,GNua |
| 0x1294B | 7 | D,GNu?< |
| 0x1296A | 5 | ,GNu. |
| 0x12978 | 5 | ,GNua |
| 0x12989 | 6 | n,GNu. |
| 0x12998 | 5 | ,GNua |
| 0x129AA | 5 | ,GNua |
| 0x129BC | 5 | ,GNua |
| 0x129CE | 5 | ,GNua |
| 0x12A56 | 5 | ,GNua |
| 0x12A68 | 5 | ,GNua |
| 0x12A7A | 5 | ,GNua |
| 0x12A8C | 5 | ,GNu3 |
| 0x12AEB | 6 | d,GNua |
| 0x12AFE | 5 | ,GNua |
| 0x12B10 | 5 | ,GNua |
| 0x12B21 | 6 | ^,GNua |
| 0x12B33 | 6 | \,GNua |
| 0x12B46 | 5 | ,GNua |
| 0x12B5A | 5 | ,GNua |
| 0x12B6C | 5 | ,GNuN |
| 0x12B7D | 6 | b,GNua |
| 0x12B8F | 6 | X,GNua |
| 0x12BA1 | 6 | h,GNua |
| 0x12BB4 | 5 | ,GNua |
| 0x12BC5 | 6 | R,GNua |
| 0x12BD8 | 5 | ,GNua |
| 0x12BE9 | 6 | V,GNua |
| 0x12BFB | 6 | \|,GNua |
| 0x12C0D | 6 | v,GNuN |
| 0x12C16 | 5 |  A"B. |
| 0x12C23 | 6 | p,GNua |
| 0x12C35 | 6 | j,GNua |
| 0x12C47 | 6 |  ,GNua |
| 0x12C5B | 6 | &,GNua |
| 0x12C6E | 5 | ,GNua |
| 0x12C80 | 5 | ,GNua |
| 0x12C92 | 5 | ,GNua |
| 0x12CA4 | 5 | ,GNua |
| 0x12CB6 | 5 | ,GNua |
| 0x12CC8 | 5 | ,GNua |
| 0x12CDC | 5 | ,GNua |
| 0x12CEE | 5 | ,GNua |
| 0x12D02 | 5 | ,GNua |
| 0x12D16 | 5 | ,GNua |
| 0x12D28 | 5 | ,GNua |
| 0x12D3A | 5 | ,GNup |
| 0x12D76 | 5 | ,GNup |
| 0x12D90 | 5 | ,GNua |
| 0x12DA1 | 6 | j,GNua |
| 0x12DB4 | 5 | ,GNua |
| 0x12DC5 | 6 | d,GNua |
| 0x12DD8 | 5 | ,GNua |
| 0x12DE9 | 6 | R,GNua |
| 0x12DFC | 5 | ,GNua |
| 0x12E0E | 5 | ,GNua |
| 0x12E20 | 5 | ,GNua |
| 0x12E32 | 5 | ,GNua |
| 0x12E43 | 6 | X,GNua |
| 0x12E55 | 6 | p,GNua |
| 0x12E68 | 5 | ,GNua |
| 0x12E7A | 5 | ,GNua |
| 0x12E8C | 5 | ,GNua |
| 0x12E9D | 6 | ^,GNua |
| 0x12EB0 | 5 | ,GNua |
| 0x12EC1 | 6 | v,GNua |
| 0x12ED4 | 5 | ,GNua |
| 0x12EE6 | 5 | ,GNua |
| 0x12EF7 | 6 | \|,GNua |
| 0x12F0E | 7 | ,GNu..- |
| 0x12F20 | 5 | ,GNua |
| 0x12F36 | 5 | ,GNua |
| 0x12F4C | 5 | ,GNua |
| 0x12F62 | 5 | ,GNua |
| 0x12F78 | 5 | ,GNua |
| 0x12F8E | 5 | ,GNua |
| 0x12FA4 | 5 | ,GNua |
| 0x12FAB | 4 | ...- |
| 0x12FBA | 5 | ,GNua |
| 0x12FC1 | 4 | n..- |
| 0x12FD0 | 5 | ,GNua |
| 0x12FD7 | 4 | 2..- |
| 0x12FE6 | 5 | ,GNua |
| 0x12FFC | 5 | ,GNua |
| 0x13012 | 5 | ,GNua |
| 0x13028 | 7 | ,GNupXN |
| 0x1304A | 5 | "@#n. |
| 0x13066 | 6 | $@p0"< |
| 0x13084 | 4 | r0!A |
| 0x130B1 | 4 | X..- |
| 0x130C0 | 5 | ,GNuN |
| 0x130D6 | 5 | ,GNua |
| 0x130EC | 5 | ,GNua |
| 0x13102 | 5 | ,GNua |
| 0x13118 | 5 | ,GNuN |
| 0x1312E | 5 | ,GNua |
| 0x13144 | 5 | ,GNuN |
| 0x1314C | 5 |  @"A" |
| 0x13152 | 5 | $C&D. |
| 0x13162 | 5 | ,GNua |
| 0x13178 | 5 | ,GNua |
| 0x13192 | 5 | ,GNua |
| 0x131AA | 6 | ,GNu"h |
| 0x132FF | 6 | dg4J@g |
| 0x13335 | 5 | 0"_!I |
| 0x1337B | 6 | 0g6`L0 |
| 0x133BE | 4 | "W2( |
| 0x1345C | 4 | "_#H |
| 0x13476 | 4 | "_#H |
| 0x13490 | 4 | "_#H |
| 0x134A3 | 4 | b"O2 |
| 0x134FD | 5 | <b^1\| |
| 0x13581 | 18 | pg@Q@k$g*Q@kNg<Q@k |
| 0x13594 | 5 | g.Q@k |
| 0x135A2 | 4 | `>Hy |
| 0x135A9 | 5 | X`6Hy |
| 0x135B2 | 4 | `.Hy |
| 0x135BA | 4 | `&Hy |
| 0x135E1 | 4 | # _O |
| 0x135F2 | 4 | "_#H |
| 0x13616 | 4 | "_#H |
| 0x13819 | 4 | DNua |
| 0x13B04 | 7 | p/ L&HJ |
| 0x13B18 | 7 |  L&Lp:J |
| 0x13B26 | 4 | &HB+ |
| 0x13B36 | 5 | p: LJ |
| 0x13B78 | 9 | `"p/ L"HJ |
| 0x13B9F | 4 | pNu/ |
| 0x13CB3 | 4 | Jmrt |
| 0x13D92 | 5 | p<Y@g |
| 0x13E40 | 4 | !`-@ |
| 0x13ECE | 4 | "@$) |
| 0x13F80 | 4 | grJn |
| 0x13FB5 | 4 | *g"A |
| 0x14066 | 4 | )f0. |
| 0x140A4 | 5 | UFgR` |
| 0x140E5 | 4 | /fNa |
| 0x1414D | 4 | Z"NN |
| 0x14174 | 5 | N]J./ |
| 0x141A4 | 5 | NupXN |
| 0x142AA | 6 | g0"@ i |
| 0x142BA | 4 | f ") |
| 0x142EC | 4 | g@2. |
| 0x142FC | 4 |  @"n |
| 0x143F4 | 4 | a" n |
| 0x14414 | 5 | N]NuH |
| 0x1442A | 4 | `,0. |
| 0x144E7 | 4 | D`Z* |
| 0x14543 | 4 | DJDn |
| 0x1454B | 5 | pNuHn |
| 0x14571 | 4 | !Nu0 |
| 0x145AA | 9 | S@SARBRCA |
| 0x14622 | 4 | ,@Bn |
| 0x146BE | 5 | VXTX` |
| 0x147F1 | 4 | ($HN |
| 0x14856 | 4 | Nu , |
| 0x1487F | 4 | :Nup |
| 0x1488D | 7 | bg<$H8. |
| 0x14905 | 6 | $ab-n( |
| 0x14949 | 5 | ~"N < |
| 0x1497E | 5 | HP @a |
| 0x14984 | 5 |  _SWj |
| 0x149C5 | 4 | 8.dA |
| 0x149D0 | 5 | .fpK2 |
| 0x149DA | 18 | NuGFA-BASIC Editor |
| 0x149ED | 14 | console.device |
| 0x14A85 | 6 | ~bbbb~ |
| 0x14A8D | 6 | ~bbbb~ |
| 0x14A95 | 6 | ~bbbb~ |
| 0x14A9D | 6 | ~bbbb~ |
| 0x14AA5 | 6 | ~bbbb~ |
| 0x14AAD | 6 | ~bbbb~ |
| 0x14AB5 | 6 | ~bbbb~ |
| 0x14ABD | 6 | ~bbbb~ |
| 0x14AC5 | 6 | ~bbbb~ |
| 0x14ACD | 6 | ~bbbb~ |
| 0x14AD5 | 6 | ~bbbb~ |
| 0x14ADC | 7 | <fn~vf< |
| 0x14B0D | 6 | 0`\|ff< |
| 0x14B1C | 7 | <ff<ff< |
| 0x14B24 | 4 | <ff> |
| 0x14B3D | 6 | ~bbbb~ |
| 0x14B45 | 6 | ~bbbb~ |
| 0x14B4D | 6 | ~bbbb~ |
| 0x14B55 | 6 | ~bbbb~ |
| 0x14B8C | 4 | 8lhv |
| 0x14BDC | 7 | <fn~vf< |
| 0x14C0D | 6 | 0`\|ff< |
| 0x14C1C | 7 | <ff<ff< |
| 0x14C24 | 4 | <ff> |
| 0x14C65 | 4 | <<f~ |
| 0x14C6D | 5 | ff\|ff |
| 0x14C7D | 5 | lfffl |
| 0x14C85 | 5 | f`x`f |
| 0x14C8D | 5 | f`x`` |
| 0x14C9C | 7 | fff~fff |
| 0x14CB5 | 5 | flxlf |
| 0x14CBD | 5 | ```bf |
| 0x14CDD | 5 | ff\|`` |
| 0x14CED | 5 | ff\|lf |
| 0x14CF4 | 4 | <fp8 |
| 0x14D04 | 7 | ffffff> |
| 0x14D0E | 4 | ff<< |
| 0x14D34 | 7 | <00000< |
| 0x14D6D | 6 | `lvff< |
| 0x14D76 | 5 | <f`f< |
| 0x14D7E | 5 | 6nff; |
| 0x14D86 | 5 | <f~`< |
| 0x14D8D | 6 | 60x00x |
| 0x14D96 | 4 | ;ff< |
| 0x14D9D | 5 | `lvff |
| 0x14DB5 | 5 | `flxl |
| 0x14DC6 | 5 | fwkcc |
| 0x14DCE | 5 | \|ffff |
| 0x14DD6 | 5 | <fff< |
| 0x14DDF | 4 | ff\|` |
| 0x14DE6 | 4 | =ff> |
| 0x14E06 | 5 | ffff; |
| 0x14E0E | 4 | fff< |
| 0x14E16 | 5 | ckk66 |
| 0x14E26 | 4 | fff< |
| 0x14E5D | 6 | ~bbbb~ |
| 0x14E65 | 6 | ~bbbb~ |
| 0x14E6D | 6 | ~bbbb~ |
| 0x14E75 | 6 | ~bbbb~ |
| 0x14E7D | 6 | ~bbbb~ |
| 0x14E85 | 6 | ~bbbb~ |
| 0x14E8D | 6 | ~bbbb~ |
| 0x14E95 | 6 | ~bbbb~ |
| 0x14E9D | 6 | ~bbbb~ |
| 0x14EA5 | 6 | ~bbbb~ |
| 0x14EAD | 6 | ~bbbb~ |
| 0x14EB5 | 6 | ~bbbb~ |
| 0x14EBD | 6 | ~bbbb~ |
| 0x14EC5 | 6 | ~bbbb~ |
| 0x14ECD | 6 | ~bbbb~ |
| 0x14ED5 | 6 | ~bbbb~ |
| 0x14EDD | 6 | ~bbbb~ |
| 0x14EE5 | 6 | ~bbbb~ |
| 0x14EED | 6 | ~bbbb~ |
| 0x14EF5 | 6 | ~bbbb~ |
| 0x14EFD | 6 | ~bbbb~ |
| 0x14F05 | 6 | ~bbbb~ |
| 0x14F6D | 4 | >ll> |
| 0x14F75 | 6 | 60x00~ |
| 0x14F7C | 5 | B<f<B |
| 0x14F94 | 5 | <@<f< |
| 0x15023 | 6 | 00p000 |
| 0x1503C | 4 |  c&, |
| 0x15044 | 4 |  c&, |
| 0x15057 | 4 | 0`f< |
| 0x1506D | 4 | $<f~ |
| 0x15084 | 5 | <f<f~ |
| 0x1508D | 4 | <<o\| |
| 0x15126 | 5 | ffff> |
| 0x1512E | 5 | ffff> |
| 0x15135 | 6 | $ffff> |
| 0x1513E | 5 | ffff> |
| 0x1514D | 6 | `~cc~` |
| 0x15154 | 9 | \|fflffl`0 |
| 0x15196 | 5 | <f`f< |
| 0x1519E | 5 | <f~`< |
| 0x151A6 | 5 | <f~`< |
| 0x151AD | 6 | $<f~`< |
| 0x151B6 | 5 | <f~`< |
| 0x151E6 | 5 | \|ffff |
| 0x151EE | 5 | <fff< |
| 0x151F6 | 5 | <fff< |
| 0x151FD | 6 | $<fff< |
| 0x15206 | 5 | <fff< |
| 0x1520E | 5 | <fff< |
| 0x1521E | 7 | >gks>@0 |
| 0x15226 | 5 | ffff; |
| 0x1522E | 5 | ffff; |
| 0x15235 | 6 | $ffff; |
| 0x1523E | 5 | ffff; |
| 0x15246 | 4 | fff< |
| 0x1524D | 6 | `\|ff\|` |
| 0x15256 | 4 | fff< |
| 0x152AC | 9 | GFA-BASIC |
| 0x152EA | 4 |  40. |
| 0x15306 | 4 |  @!N |
| 0x1532C | 6 | VB.&-\| |
| 0x15336 | 4 | . -\| |
| 0x15380 | 5 |  @!n- |
| 0x15391 | 4 | J n( |
| 0x15399 | 4 | 2-I- |
| 0x153AE | 4 | -@.` |
| 0x153DB | 4 | p-@( |
| 0x15404 | 4 | Nu < |
| 0x15411 | 4 | ` n- |
| 0x15445 | 4 | `dVA |
| 0x1545F | 6 | n.dfBA |
| 0x1546C | 4 | $D!R |
| 0x15486 | 5 | ,G"_/ |
| 0x154A5 | 4 | vBg? |
| 0x154BC | 5 | f"-E. |
| 0x154C9 | 4 | R=A. |
| 0x15518 | 4 | &I#H |
| 0x15548 | 4 | Nu"< |
| 0x15558 | 7 | gL$@%n. |
| 0x1556A | 4 | Nup0 |
| 0x15584 | 4 | g  @ |
| 0x15592 | 4 | r0!A |
| 0x155B0 | 5 | "Jp"` |
| 0x155DC | 5 | Nu"n( |
| 0x15605 | 4 | T n( |
| 0x1560D | 13 | :input.device |
| 0x1561E | 4 | `J"y |
| 0x15656 | 5 | "i. 2 |
| 0x1565C | 4 | Nq1\| |
| 0x1566A | 5 | NuNqH |
| 0x15679 | 4 | X @L |
| 0x15682 | 6 | "./Jg@ |
| 0x15690 | 5 | /^e4A |
| 0x156BA | 5 | "n. 2 |
| 0x156C8 | 6 | "./Rg" |
| 0x156E8 | 5 | "n. 2 |
| 0x156FC | 5 | -@..a |
| 0x15704 | 5 | ax ./ |
| 0x1570E | 5 | gh-@/ |
| 0x15744 | 4 | B@H@ |
| 0x1574E | 4 | B@H@ |
| 0x15758 | 4 | B@H@ |
| 0x1576A | 5 |  ..`g |
| 0x1577C | 7 | Nu"n.`E |
| 0x157C4 | 4 | B@H@ |
| 0x157D4 | 4 | B@H@ |
| 0x157E4 | 4 | B@H@ |
| 0x157F0 | 4 | B@H@ |
| 0x15800 | 4 | B@H@ |
| 0x15826 | 4 | Nu6< |
| 0x15836 | 5 | -@.6a |
| 0x1583D | 6 | 0-@.:A |
| 0x15848 | 4 | .T!I |
| 0x15860 | 5 | "n.:p |
| 0x1586C | 6 | "n.:3C |
| 0x15876 | 4 | .>#H |
| 0x1587F | 6 | $"n.:a |
| 0x15888 | 5 | "n.:a |
| 0x1588F | 6 | *"n.6` |
| 0x158A0 | 5 |  @!n. |
| 0x158A7 | 6 | xNu&H& |
| 0x158DB | 8 | ,Nu"H$HJ |
| 0x15903 | 4 | ."Lt |
| 0x1594A | 5 | SC$AG |
| 0x15987 | 6 | )PB$KQ |
| 0x1599C | 4 | "L4< |
| 0x159A8 | 5 | RCr(a |
| 0x159AF | 4 | h=C" |
| 0x159FA | 4 | j6XG |
| 0x15A48 | 4 | #r%K |
| 0x15A50 | 5 | #*"Ha |
| 0x15A5A | 4 | #:#H |
| 0x15A66 | 4 | #J#H |
| 0x15A6E | 4 | #^#H |
| 0x15A77 | 4 | E8." |
| 0x15B00 | 5 | &IRB` |
| 0x15B2C | 5 | (H<." |
| 0x15B61 | 4 | X Kp |
| 0x15B82 | 7 | =@#v=A# |
| 0x15BF0 | 8 | "@&@ J!\| |
| 0x15C1B | 4 | N Ka |
| 0x15C3E | 4 | HC*) |
| 0x15C62 | 4 |  E8( |
| 0x15C67 | 6 | & J"Ka |
| 0x15C88 | 4 | #r<( |
| 0x15CBF | 4 | FNuB |
| 0x15CEA | 9 | #:-H#6NuI |
| 0x15D06 | 5 | SENup |
| 0x15D0C | 5 |  KBXQ |
| 0x15D20 | 5 | SCk4p |
| 0x15D2E | 4 | SDk( |
| 0x15D3A | 4 | g R@ |
| 0x15D4B | 4 | \|SDB |
| 0x15D67 | 4 | FNur |
| 0x15D7B | 4 | @ .' |
| 0x15DA8 | 7 | "@Nu .' |
| 0x15DB8 | 4 |  @"h |
| 0x15DBD | 6 | 2Nu n' |
| 0x15DC7 | 6 | TNuHPa |
| 0x15DCF | 4 | \|-@' |
| 0x15DD4 | 5 | gd&@a |
| 0x15DDB | 4 | p-@' |
| 0x15DF2 | 7 | $H"@ n' |
| 0x15E00 | 5 | f8 _L |
| 0x15E44 | 5 | g@ n' |
| 0x15E69 | 4 | P"n' |
| 0x15E71 | 4 | :"n' |
| 0x15E96 | 4 |  @,h |
| 0x15E9B | 4 | XNuH |
| 0x15EC1 | 5 | V"W3\| |
| 0x15EE9 | 6 | h"_p`N |
| 0x15EF8 | 14 | Nutimer.device |
| 0x15F3B | 5 | ALERT |
| 0x15F4D | 8 | ABSOLUTE |
| 0x15F5A | 5 | AFTER |
| 0x15F64 | 5 | AUDIO |
| 0x15F6E | 9 | ARRAYFILL |
| 0x15F7C | 5 | BMOVE |
| 0x15F86 | 5 | BYTE{ |
| 0x15F98 | 4 | BGET |
| 0x15FA1 | 4 | BPUT |
| 0x15FAA | 8 | BOUNDARY |
| 0x15FB7 | 5 | BSAVE |
| 0x15FC1 | 5 | BLOAD |
| 0x15FCB | 5 | BACKW |
| 0x15FD5 | 5 | BACKS |
| 0x15FDF | 5 | COLOR |
| 0x15FE9 | 4 | CASE |
| 0x15FF2 | 4 | CONT |
| 0x15FFB | 5 | CARD{ |
| 0x16005 | 5 | CHAR{ |
| 0x1600F | 5 | CLOSE |
| 0x16019 | 5 | CLEAR |
| 0x16023 | 6 | CIRCLE |
| 0x1602E | 7 | CHDRIVE |
| 0x1603A | 5 | CHDIR |
| 0x16054 | 4 | CLIP |
| 0x1605D | 5 | CHAIN |
| 0x16067 | 4 | CALL |
| 0x16070 | 4 | CEND |
| 0x16079 | 5 | CINIT |
| 0x16083 | 5 | CMOVE |
| 0x1608D | 5 | CWAIT |
| 0x16097 | 5 | CURVE |
| 0x160A1 | 4 | DATA |
| 0x160B1 | 7 | DEFLINE |
| 0x160BD | 8 | DEFMOUSE |
| 0x160CA | 7 | DEFLIST |
| 0x160D6 | 7 | DEFMARK |
| 0x160E2 | 6 | DEFNUM |
| 0x160ED | 7 | DEFTEXT |
| 0x160F9 | 7 | DEFFILL |
| 0x16105 | 5 | DEFFN |
| 0x1610F | 7 | DEFAULT |
| 0x1612B | 5 | DPOKE |
| 0x1613D | 7 | DOUBLE{ |
| 0x16151 | 6 | DELETE |
| 0x1615C | 4 | DRAW |
| 0x16165 | 5 | DELAY |
| 0x1616F | 6 | DEFINT |
| 0x1617A | 6 | DEFBYT |
| 0x16185 | 6 | DEFWRD |
| 0x16190 | 6 | DEFBIT |
| 0x1619B | 6 | DEFSTR |
| 0x161A6 | 6 | DEFSNG |
| 0x161B1 | 6 | DEFDBL |
| 0x161BC | 6 | DEFFLT |
| 0x161C7 | 4 | DUMP |
| 0x161D0 | 6 | DATE$= |
| 0x161DB | 7 | DISPLAY |
| 0x161E7 | 4 | ELSE |
| 0x161F0 | 6 | END IF |
| 0x161FB | 5 | ENDIF |
| 0x16205 | 7 | ENDFUNC |
| 0x16211 | 9 | ENDSELECT |
| 0x1621F | 8 | ENDWHILE |
| 0x1622C | 5 | ENDDO |
| 0x16236 | 9 | ENDREPEAT |
| 0x16244 | 6 | ENDFOR |
| 0x1624F | 7 | ENDPROC |
| 0x1625B | 6 | ENDSUB |
| 0x16266 | 7 | EXIT IF |
| 0x16272 | 7 | EXIT IF |
| 0x1627E | 7 | ELLIPSE |
| 0x1628A | 5 | ERROR |
| 0x16294 | 4 | EDIT |
| 0x1629D | 5 | ERASE |
| 0x162A7 | 4 | EXEC |
| 0x162B0 | 5 | EVERY |
| 0x162C2 | 8 | FUNCTION |
| 0x162CF | 6 | FLOAT{ |
| 0x162DA | 4 | FILL |
| 0x162E3 | 5 | FIELD |
| 0x162ED | 5 | FILES |
| 0x162F7 | 10 | FILESELECT |
| 0x16306 | 5 | FULLW |
| 0x16310 | 6 | FRONTW |
| 0x1631B | 6 | FRONTS |
| 0x16326 | 4 | GURU |
| 0x1632F | 5 | GOSUB |
| 0x16339 | 4 | GOTO |
| 0x1634A | 9 | GRAPHMODE |
| 0x16358 | 8 | HARDCOPY |
| 0x16365 | 4 | HTAB |
| 0x1637D | 4 | INT{ |
| 0x16386 | 6 | INSERT |
| 0x16391 | 5 | INPUT |
| 0x1639B | 6 | INLINE |
| 0x163A6 | 4 | KILL |
| 0x163AF | 4 | LOOP |
| 0x163C0 | 4 | LINE |
| 0x163C9 | 5 | LPOKE |
| 0x163D3 | 5 | LONG{ |
| 0x163DD | 5 | LOCAL |
| 0x163E7 | 6 | LPRINT |
| 0x163F2 | 4 | LSET |
| 0x163FB | 6 | LOCATE |
| 0x16406 | 4 | LIST |
| 0x1640F | 5 | LLIST |
| 0x16419 | 4 | LOAD |
| 0x16422 | 6 | LIMITW |
| 0x16435 | 7 | MONITOR |
| 0x16449 | 4 | MENU |
| 0x16452 | 5 | MID$( |
| 0x1645C | 5 | MKDIR |
| 0x16466 | 5 | MOUSE |
| 0x16470 | 4 | MODE |
| 0x16479 | 5 | MOVEW |
| 0x16483 | 5 | MOVES |
| 0x1648D | 4 | NEXT |
| 0x16496 | 4 | NAME |
| 0x164A7 | 4 | OPEN |
| 0x164BF | 9 | OTHERWISE |
| 0x164CD | 6 | OPTION |
| 0x164D8 | 4 | O.AX |
| 0x164E1 | 4 | O.AY |
| 0x164FA | 4 | O.VX |
| 0x16503 | 4 | O.VY |
| 0x1650C | 6 | O.CLIP |
| 0x16517 | 7 | O.CLOSE |
| 0x16523 | 4 | O.ON |
| 0x1652C | 5 | O.OFF |
| 0x16536 | 5 | O.HIT |
| 0x16540 | 8 | O.PLANES |
| 0x1654D | 10 | O.PRIORITY |
| 0x1655C | 7 | O.SHAPE |
| 0x16568 | 7 | O.START |
| 0x16574 | 6 | O.STOP |
| 0x1657F | 9 | OBJECT.AX |
| 0x1658D | 9 | OBJECT.AY |
| 0x1659B | 8 | OBJECT.X |
| 0x165A8 | 8 | OBJECT.Y |
| 0x165B5 | 9 | OBJECT.VX |
| 0x165C3 | 9 | OBJECT.VY |
| 0x165D1 | 11 | OBJECT.CLIP |
| 0x165E1 | 12 | OBJECT.CLOSE |
| 0x165F2 | 9 | OBJECT.ON |
| 0x16600 | 10 | OBJECT.OFF |
| 0x1660F | 10 | OBJECT.HIT |
| 0x1661E | 13 | OBJECT.PLANES |
| 0x16630 | 15 | OBJECT.PRIORITY |
| 0x16644 | 12 | OBJECT.SHAPE |
| 0x16655 | 12 | OBJECT.START |
| 0x16666 | 11 | OBJECT.STOP |
| 0x16676 | 5 | PRINT |
| 0x16680 | 6 | PCOLOR |
| 0x1668B | 4 | PLOT |
| 0x16694 | 4 | POKE |
| 0x1669D | 9 | PROCEDURE |
| 0x166B3 | 4 | PBOX |
| 0x166BC | 5 | PRBOX |
| 0x166C6 | 7 | PCIRCLE |
| 0x166D2 | 8 | PELLIPSE |
| 0x166DF | 5 | PAUSE |
| 0x166E9 | 8 | POLYLINE |
| 0x166F6 | 8 | POLYFILL |
| 0x16703 | 8 | POLYMARK |
| 0x16710 | 5 | PSAVE |
| 0x1671A | 4 | QUIT |
| 0x16723 | 5 | QSORT |
| 0x16735 | 6 | REPEAT |
| 0x16740 | 7 | RESTORE |
| 0x1674C | 6 | RETURN |
| 0x16757 | 6 | RETURN |
| 0x16762 | 7 | RELSEEK |
| 0x1676E | 7 | RESERVE |
| 0x1677A | 9 | RANDOMIZE |
| 0x16788 | 4 | RBOX |
| 0x16791 | 5 | RMDIR |
| 0x1679B | 4 | RSET |
| 0x167A4 | 6 | RENAME |
| 0x167AF | 5 | RCALL |
| 0x167B9 | 6 | RESUME |
| 0x167C4 | 4 | READ |
| 0x167CD | 6 | RECORD |
| 0x167E0 | 6 | RECALL |
| 0x167EB | 8 | RASTPORT |
| 0x167F8 | 6 | SELECT |
| 0x16803 | 4 | SWAP |
| 0x16814 | 5 | SPOKE |
| 0x1681E | 6 | SDPOKE |
| 0x16829 | 6 | SLPOKE |
| 0x16834 | 8 | SETCOLOR |
| 0x16841 | 4 | SEEK |
| 0x1684A | 7 | SETTIME |
| 0x16856 | 4 | SGET |
| 0x1685F | 4 | SPUT |
| 0x16868 | 4 | STOP |
| 0x16871 | 6 | SYSTEM |
| 0x1687C | 5 | SSORT |
| 0x16886 | 5 | STICK |
| 0x16890 | 10 | SOUND WAIT |
| 0x1689F | 12 | SOUND RESUME |
| 0x168B0 | 5 | SOUND |
| 0x168BA | 7 | SINGLE{ |
| 0x168C6 | 4 | SAVE |
| 0x168CF | 7 | SETDRAW |
| 0x168DB | 5 | STORE |
| 0x168E5 | 6 | SPRITE |
| 0x168F0 | 8 | SPRITE # |
| 0x168FD | 5 | SIZEW |
| 0x16907 | 7 | SETSPEN |
| 0x16913 | 7 | SETWPEN |
| 0x1691F | 9 | SETSTITLE |
| 0x1692D | 9 | SETWTITLE |
| 0x1693B | 5 | SLEEP |
| 0x1694D | 6 | SCROLL |
| 0x16958 | 4 | TEXT |
| 0x16961 | 4 | TRON |
| 0x1696A | 5 | TROFF |
| 0x16974 | 4 | TOPW |
| 0x1697D | 6 | TITLEW |
| 0x16988 | 6 | TITLES |
| 0x16993 | 5 | TOUCH |
| 0x1699D | 5 | UNTIL |
| 0x169A7 | 4 | VOID |
| 0x169B0 | 5 | VSYNC |
| 0x169BA | 4 | VTAB |
| 0x169C3 | 5 | WHILE |
| 0x169CD | 4 | WEND |
| 0x169D6 | 4 | WAVE |
| 0x169DF | 5 | WORD{ |
| 0x169E9 | 5 | WRITE |
| 0x16A43 | 6 | _DATA= |
| 0x16A6C | 5 | OPENW |
| 0x16A76 | 5 | OPENS |
| 0x16A80 | 6 | CLOSES |
| 0x16A8B | 10 | FORM INPUT |
| 0x16A9A | 10 | LINE INPUT |
| 0x16AA9 | 7 | ELSE IF |
| 0x16ABD | 8 | DO WHILE |
| 0x16ACA | 8 | DO UNTIL |
| 0x16AD7 | 10 | LOOP WHILE |
| 0x16AE6 | 10 | LOOP UNTIL |
| 0x16AF5 | 8 | ON ERROR |
| 0x16B02 | 14 | ON ERROR GOSUB |
| 0x16B15 | 8 | ON BREAK |
| 0x16B22 | 13 | ON BREAK CONT |
| 0x16B34 | 14 | ON BREAK GOSUB |
| 0x16B47 | 13 | ON MENU GOSUB |
| 0x16B59 | 16 | ON MESSAGE GOSUB |
| 0x16B6E | 17 | ON MENU KEY GOSUB |
| 0x16B84 | 20 | ON MENU BUTTON GOSUB |
| 0x16B9D | 18 | ON COLLISION GOSUB |
| 0x16BB4 | 7 | ON MENU |
| 0x16BC0 | 9 | MENU KILL |
| 0x16BCE | 4 | MENU |
| 0x16BD7 | 7 | OPENW # |
| 0x16BE3 | 8 | CLOSEW # |
| 0x16BF0 | 11 | > PROCEDURE |
| 0x16C00 | 10 | > FUNCTION |
| 0x16C0F | 9 | RELSEEK # |
| 0x16C1D | 6 | SEEK # |
| 0x16C28 | 8 | TITLEW # |
| 0x16C35 | 8 | TITLES # |
| 0x16C42 | 7 | MOVEW # |
| 0x16C4E | 7 | SIZEW # |
| 0x16C5A | 8 | CLEARW # |
| 0x16C67 | 7 | BACKW # |
| 0x16C73 | 7 | FULLW # |
| 0x16C7F | 8 | LIMITW # |
| 0x16C8C | 8 | FRONTW # |
| 0x16C99 | 7 | TOUCH # |
| 0x16CA5 | 7 | DEFAULT |
| 0x16CB1 | 6 | SELECT |
| 0x16CBC | 5 | OUT # |
| 0x16CC6 | 7 | MAT ADD |
| 0x16CD2 | 7 | MAT SUB |
| 0x16CDE | 7 | MAT CPY |
| 0x16CEA | 8 | MAT XCPY |
| 0x16CF7 | 7 | MAT DET |
| 0x16D03 | 7 | MAT NEG |
| 0x16D0F | 7 | MAT ABS |
| 0x16D1B | 8 | MAT NORM |
| 0x16D28 | 8 | MAT READ |
| 0x16D35 | 9 | MAT PRINT |
| 0x16D43 | 9 | MAT TRANS |
| 0x16D51 | 7 | MAT CLR |
| 0x16D5D | 7 | MAT SET |
| 0x16D69 | 7 | MAT ONE |
| 0x16D75 | 8 | MAT BASE |
| 0x16D82 | 8 | MAT QDET |
| 0x16D8F | 9 | MAT INPUT |
| 0x16D9D | 8 | MAT RANG |
| 0x16DAA | 7 | MAT MUL |
| 0x16DB6 | 7 | MAT INV |
| 0x16E8E | 4 | AND( |
| 0x16E95 | 4 | ABS( |
| 0x16E9C | 4 | ATN( |
| 0x16EA3 | 4 | ASC( |
| 0x16EB0 | 5 | ASIN( |
| 0x16EB8 | 5 | ACOS( |
| 0x16EC0 | 8 | AskFont( |
| 0x16ECB | 13 | AskSoftStyle( |
| 0x16EE0 | 7 | ARRPTR( |
| 0x16EEA | 4 | ADD( |
| 0x16EF1 | 8 | AbortIO( |
| 0x16EFC | 15 | ActivateGadget( |
| 0x16F0E | 15 | ActivateWindow( |
| 0x16F20 | 10 | AddAnimOb( |
| 0x16F2D | 7 | AddBob( |
| 0x16F37 | 10 | AddDevice( |
| 0x16F44 | 8 | AddFont( |
| 0x16F4F | 12 | AddFreeList( |
| 0x16F5E | 10 | AddGadget( |
| 0x16F6B | 9 | AddGList( |
| 0x16F77 | 8 | AddHead( |
| 0x16F82 | 13 | AddIntServer( |
| 0x16F92 | 11 | AddLibrary( |
| 0x16FA0 | 8 | AddPort( |
| 0x16FAB | 12 | AddResource( |
| 0x16FBA | 8 | AddTail( |
| 0x16FC5 | 8 | AddTask( |
| 0x16FD0 | 8 | AddTime( |
| 0x16FDB | 11 | AddVSprite( |
| 0x16FE9 | 6 | Alert( |
| 0x16FF2 | 9 | AllocAbs( |
| 0x16FFE | 9 | Allocate( |
| 0x1700A | 11 | AllocEntry( |
| 0x17018 | 9 | AllocMem( |
| 0x17024 | 13 | AllocPotBits( |
| 0x17034 | 12 | AllocRaster( |
| 0x17043 | 14 | AllocRemember( |
| 0x17054 | 12 | AllocSignal( |
| 0x17063 | 10 | AllocTrap( |
| 0x17070 | 14 | AllocWBObject( |
| 0x17081 | 15 | AlohaWorkbench( |
| 0x17093 | 14 | AndRectRegion( |
| 0x170A4 | 16 | AndRegionRegion( |
| 0x170BD | 8 | Animate( |
| 0x170C8 | 11 | AreaCircle( |
| 0x170D6 | 9 | AreaDraw( |
| 0x170E2 | 12 | AreaEllipse( |
| 0x170F1 | 8 | AreaEnd( |
| 0x170FC | 9 | AreaMove( |
| 0x17108 | 20 | AttemptLockLayerRom( |
| 0x1711F | 12 | AutoRequest( |
| 0x1712E | 4 | AUTO |
| 0x17135 | 11 | AvailFonts( |
| 0x17143 | 9 | AvailMem( |
| 0x1714F | 5 | BIN$( |
| 0x17157 | 5 | BREAK |
| 0x1715F | 6 | BUTTON |
| 0x17168 | 4 | BASE |
| 0x1716F | 5 | BYTE( |
| 0x17177 | 5 | BTST( |
| 0x1717F | 5 | BSET( |
| 0x17187 | 5 | BCLR( |
| 0x1718F | 5 | BCHG( |
| 0x17197 | 5 | BYTE{ |
| 0x1719F | 13 | BeginRefresh( |
| 0x171AF | 12 | BeginUpdate( |
| 0x171BE | 12 | BehindLayer( |
| 0x171CD | 10 | BltBitMap( |
| 0x171DA | 18 | BltBitMapRastPort( |
| 0x171EF | 9 | BltClear( |
| 0x171FB | 22 | BltMaskBitMapRastPort( |
| 0x17214 | 11 | BltPattern( |
| 0x17222 | 12 | BltTemplate( |
| 0x17231 | 16 | BuildSysRequest( |
| 0x17244 | 13 | BumpRevision( |
| 0x17254 | 4 | COS( |
| 0x1725B | 5 | COSQ( |
| 0x17263 | 5 | CHR$( |
| 0x1726B | 4 | CVI( |
| 0x17272 | 4 | CVL( |
| 0x17279 | 4 | CVS( |
| 0x17280 | 4 | CVF( |
| 0x17287 | 4 | CVD( |
| 0x1728E | 4 | CONT |
| 0x1729A | 6 | CRSLIN |
| 0x172A3 | 6 | CRSCOL |
| 0x172AC | 5 | CARD( |
| 0x172B4 | 5 | CARD{ |
| 0x172BC | 5 | CHAR{ |
| 0x172C4 | 5 | CINT( |
| 0x172CC | 7 | CFLOAT( |
| 0x172D6 | 6 | Cause( |
| 0x172DF | 6 | CBump( |
| 0x172E8 | 15 | CDInputHandler( |
| 0x172FA | 13 | ChangeSprite( |
| 0x1730A | 8 | CheckIo( |
| 0x17315 | 15 | ClearDMRequest( |
| 0x17327 | 9 | ClearEOL( |
| 0x17333 | 15 | ClearMenuStrip( |
| 0x17345 | 13 | ClearPointer( |
| 0x17355 | 16 | ClearRectRegion( |
| 0x17368 | 12 | ClearRegion( |
| 0x17377 | 12 | ClearScreen( |
| 0x17386 | 9 | ClipBlit( |
| 0x17392 | 6 | Close( |
| 0x1739B | 12 | CloseDevice( |
| 0x173AA | 10 | CloseFont( |
| 0x173B7 | 13 | CloseLibrary( |
| 0x173C7 | 12 | CloseScreen( |
| 0x173D6 | 12 | CloseWindow( |
| 0x173E5 | 16 | CloseWorkBench() |
| 0x173F8 | 6 | CMove( |
| 0x17401 | 8 | CmpTime( |
| 0x1740C | 12 | CopySBitMap( |
| 0x1741B | 18 | CreateBehindLayer( |
| 0x17430 | 10 | CreateDir( |
| 0x1743D | 12 | CreateExtIO( |
| 0x1744C | 11 | CreatePort( |
| 0x1745A | 11 | CreateProc( |
| 0x17468 | 12 | CreateStdIO( |
| 0x17477 | 19 | CreateUpfrontLayer( |
| 0x1748D | 11 | CurrentDir( |
| 0x1749B | 12 | CurrentTime( |
| 0x174AA | 6 | CWait( |
| 0x174B3 | 10 | COLLISION( |
| 0x174C0 | 9 | COLLISION |
| 0x174CC | 7 | COMBIN( |
| 0x174DC | 4 | DIV( |
| 0x174E3 | 6 | DOWNTO |
| 0x174EC | 6 | DPEEK( |
| 0x174F5 | 5 | DIM?( |
| 0x174FD | 5 | DATE$ |
| 0x17505 | 5 | DIR$( |
| 0x1750D | 6 | DFREE( |
| 0x17516 | 7 | DOUBLE{ |
| 0x17520 | 4 | DEG( |
| 0x17527 | 5 | Draw( |
| 0x1752F | 5 | DRAW( |
| 0x17537 | 10 | DateStamp( |
| 0x17544 | 11 | Deallocate( |
| 0x17552 | 7 | Debug() |
| 0x1755C | 6 | Delay( |
| 0x17565 | 11 | DeleteExtIO |
| 0x17573 | 11 | DeleteFile( |
| 0x17581 | 12 | DeleteLayer( |
| 0x17590 | 11 | DeletePort( |
| 0x1759E | 12 | DeleteStdIO( |
| 0x175AD | 11 | DeviceProc( |
| 0x175BB | 9 | Disable() |
| 0x175C7 | 15 | DisOwnBlitter() |
| 0x175D9 | 10 | Dispatch() |
| 0x175E6 | 13 | DisplayAlert( |
| 0x175F6 | 12 | DisplayBeep( |
| 0x17605 | 17 | DisposeLayerInfo( |
| 0x17619 | 14 | DisposeRegion( |
| 0x1762A | 12 | DoCollision( |
| 0x17639 | 5 | DoIO( |
| 0x17641 | 12 | DoubleClick( |
| 0x17650 | 11 | DrawBorder( |
| 0x1765E | 11 | DrawCircle( |
| 0x1766C | 12 | DrawEllipse( |
| 0x1767B | 10 | DrawGList( |
| 0x17688 | 10 | DrawImage( |
| 0x17695 | 8 | DupLock( |
| 0x176A0 | 5 | EOF(# |
| 0x176A8 | 4 | EOF( |
| 0x176B5 | 4 | EQV( |
| 0x176BC | 5 | ERROR |
| 0x176C4 | 5 | ERR$( |
| 0x176D2 | 5 | EVEN( |
| 0x176DA | 5 | EXEC( |
| 0x176E2 | 6 | EXIST( |
| 0x176EB | 4 | EXP( |
| 0x176F2 | 8 | Enable() |
| 0x176FD | 11 | EndRefresh( |
| 0x1770B | 11 | EndRequest( |
| 0x17719 | 10 | EndUpdate( |
| 0x17726 | 8 | Enqueue( |
| 0x17731 | 8 | Examine( |
| 0x1773C | 8 | Execute( |
| 0x17747 | 11 | Exception() |
| 0x17755 | 5 | Exit( |
| 0x1775D | 10 | ExitIntr() |
| 0x1776A | 7 | ExNext( |
| 0x17774 | 5 | FRAC( |
| 0x1777C | 4 | FRE( |
| 0x17783 | 5 | FALSE |
| 0x1778B | 5 | FATAL |
| 0x17793 | 4 | FIX( |
| 0x1779F | 6 | FLOAT{ |
| 0x177A8 | 16 | FattenLayerInfo( |
| 0x177BB | 9 | FindName( |
| 0x177C7 | 9 | FindPort( |
| 0x177D3 | 13 | FindResident( |
| 0x177E3 | 9 | FindTask( |
| 0x177EF | 13 | FindToolType( |
| 0x177FF | 6 | Flood( |
| 0x17808 | 8 | Forbid() |
| 0x17813 | 13 | FreeColorMap( |
| 0x17823 | 12 | FreeCopList( |
| 0x17832 | 12 | FreeCprList( |
| 0x17841 | 15 | FreeDiskObject( |
| 0x17853 | 10 | FreeEntry( |
| 0x17860 | 13 | FreeFreeList( |
| 0x17870 | 13 | FreeGBuffers( |
| 0x17880 | 8 | FreeMem( |
| 0x1788B | 12 | FreePotBits( |
| 0x1789A | 11 | FreeRaster( |
| 0x178A8 | 13 | FreeRemember( |
| 0x178B8 | 11 | FreeSignal( |
| 0x178C6 | 11 | FreeSprite( |
| 0x178D4 | 15 | FreeSysRequest( |
| 0x178E6 | 9 | FreeTrap( |
| 0x178F2 | 18 | FreeVPortCopLists( |
| 0x17907 | 13 | FreeWBObject( |
| 0x17917 | 5 | FACT( |
| 0x1791F | 4 | GURU |
| 0x17926 | 5 | GOSUB |
| 0x1792E | 4 | GOTO |
| 0x17935 | 7 | GetCC() |
| 0x1793F | 12 | GetColorMap( |
| 0x1794E | 9 | GetPrefs( |
| 0x1795A | 12 | GetDefPrefs( |
| 0x17969 | 14 | GetDiskObject( |
| 0x1797A | 12 | GetGBuffers( |
| 0x17989 | 8 | GetIcon( |
| 0x17994 | 7 | GetMsg( |
| 0x1799E | 8 | GetRGB4( |
| 0x179A9 | 14 | GetScreenData( |
| 0x179BA | 10 | GetSprite( |
| 0x179C7 | 12 | GetWBObject( |
| 0x179D6 | 5 | HEX$( |
| 0x179DE | 5 | HIMEM |
| 0x179F1 | 4 | IMP( |
| 0x179F8 | 6 | INKEY$ |
| 0x17A01 | 7 | INPUT$( |
| 0x17A0B | 7 | Input() |
| 0x17A15 | 5 | INP(# |
| 0x17A1D | 4 | INP( |
| 0x17A24 | 5 | INPUT |
| 0x17A2C | 6 | INSTR( |
| 0x17A35 | 4 | INT( |
| 0x17A3C | 4 | INT{ |
| 0x17A43 | 5 | Info( |
| 0x17A4B | 9 | InitArea( |
| 0x17A57 | 11 | InitBitMap( |
| 0x17A65 | 9 | InitCode( |
| 0x17A71 | 9 | InitGels( |
| 0x17A7D | 11 | InitGMasks( |
| 0x17A8B | 11 | InitLayers( |
| 0x17A99 | 10 | InitMasks( |
| 0x17AA6 | 13 | InitRastPort( |
| 0x17AB6 | 14 | InitRequester( |
| 0x17AC7 | 13 | InitResident( |
| 0x17AD7 | 11 | InitStruct( |
| 0x17AE5 | 11 | InitTmpRas( |
| 0x17AF3 | 9 | InitView( |
| 0x17AFF | 10 | InitVPort( |
| 0x17B0C | 7 | Insert( |
| 0x17B16 | 18 | InstallClipRegion( |
| 0x17B2B | 16 | IntuiTextLength( |
| 0x17B3E | 10 | Intuition( |
| 0x17B4B | 7 | IoErr() |
| 0x17B55 | 14 | IsInteractive( |
| 0x17B66 | 12 | ItemAddress( |
| 0x17B7B | 4 | KILL |
| 0x17B82 | 6 | LEFT$( |
| 0x17B8B | 4 | LEN( |
| 0x17B92 | 5 | LOC(# |
| 0x17B9A | 4 | LOC( |
| 0x17BA1 | 5 | LOF(# |
| 0x17BA9 | 4 | LOF( |
| 0x17BB0 | 4 | LOG( |
| 0x17BB7 | 6 | LOG10( |
| 0x17BC0 | 6 | LPEEK( |
| 0x17BCE | 5 | LPOS( |
| 0x17BD6 | 5 | LONG{ |
| 0x17BDE | 9 | LoadRGB4( |
| 0x17BEA | 8 | LoadSeg( |
| 0x17BF5 | 9 | LoadView( |
| 0x17C01 | 5 | Lock( |
| 0x17C09 | 10 | LockIBase( |
| 0x17C16 | 10 | LockLayer( |
| 0x17C23 | 14 | LockLayerInfo( |
| 0x17C34 | 13 | LockLayerRom( |
| 0x17C44 | 11 | LockLayers( |
| 0x17C52 | 4 | MAX( |
| 0x17C59 | 5 | MID$( |
| 0x17C61 | 4 | MIN( |
| 0x17C68 | 5 | MKD$( |
| 0x17C70 | 5 | MKF$( |
| 0x17C78 | 5 | MKI$( |
| 0x17C80 | 5 | MKL$( |
| 0x17C88 | 5 | MKS$( |
| 0x17C90 | 4 | MOD( |
| 0x17C97 | 6 | MOUSEK |
| 0x17CA0 | 6 | MOUSEX |
| 0x17CA9 | 6 | MOUSEY |
| 0x17CB2 | 8 | MenuNum( |
| 0x17CBD | 5 | MENU( |
| 0x17CC5 | 4 | MENU |
| 0x17CCC | 7 | MESSAGE |
| 0x17CD6 | 4 | MUL( |
| 0x17CDD | 7 | MALLOC( |
| 0x17CE7 | 6 | MFREE( |
| 0x17CF0 | 14 | MakeFunctions( |
| 0x17D01 | 12 | MakeLibrary( |
| 0x17D10 | 11 | MakeScreen( |
| 0x17D1E | 10 | MakeVPort( |
| 0x17D2B | 15 | MatchToolValue( |
| 0x17D3D | 12 | ModifyIDCMP( |
| 0x17D4C | 11 | ModifyProp( |
| 0x17D60 | 5 | Move( |
| 0x17D68 | 10 | MoveLayer( |
| 0x17D75 | 19 | MoveLayerInFrontOf( |
| 0x17D8B | 11 | MoveScreen( |
| 0x17D99 | 11 | MoveSprite( |
| 0x17DA7 | 11 | MoveWindow( |
| 0x17DB5 | 7 | MrgCop( |
| 0x17DBF | 4 | NEXT |
| 0x17DCC | 14 | NewLayerInfo() |
| 0x17DDD | 11 | NewRegion() |
| 0x17DEB | 5 | OCT$( |
| 0x17DF3 | 4 | ODD( |
| 0x17DFA | 6 | OFFSET |
| 0x17E09 | 10 | OffGadget( |
| 0x17E16 | 8 | OffMenu( |
| 0x17E27 | 15 | OldOpenLibrary( |
| 0x17E39 | 9 | OnGadget( |
| 0x17E45 | 7 | OnMenu( |
| 0x17E54 | 5 | Open( |
| 0x17E5C | 11 | OpenDevice( |
| 0x17E6A | 13 | OpenDiskFont( |
| 0x17E7A | 13 | OpenDiskFont( |
| 0x17E8A | 9 | OpenFont( |
| 0x17E96 | 12 | OpenLibrary( |
| 0x17EA5 | 13 | OpenResource( |
| 0x17EB5 | 11 | OpenScreen( |
| 0x17EC3 | 11 | OpenWindow( |
| 0x17ED1 | 15 | OpenWorkBench() |
| 0x17EE3 | 13 | OrRectRegion( |
| 0x17EF3 | 15 | OrRegionRegion( |
| 0x17F0A | 8 | Output() |
| 0x17F15 | 12 | OwnBlitter() |
| 0x17F24 | 9 | OBJECT.X( |
| 0x17F30 | 9 | OBJECT.Y( |
| 0x17F3C | 10 | OBJECT.VX( |
| 0x17F49 | 10 | OBJECT.VY( |
| 0x17F56 | 10 | OBJECT.AX( |
| 0x17F63 | 10 | OBJECT.AY( |
| 0x17F70 | 5 | PEEK( |
| 0x17F7D | 6 | POINT( |
| 0x17F86 | 4 | POS( |
| 0x17F8D | 5 | PRED( |
| 0x17F95 | 10 | ParentDir( |
| 0x17FA2 | 8 | Permit() |
| 0x17FAD | 9 | PolyDraw( |
| 0x17FB9 | 11 | PrintIText( |
| 0x17FC7 | 8 | Procure( |
| 0x17FD2 | 14 | PutDiskObject( |
| 0x17FE3 | 8 | PutIcon( |
| 0x17FEE | 7 | PutMsg( |
| 0x17FF8 | 12 | PutWBObject( |
| 0x18007 | 6 | QBlit( |
| 0x18010 | 8 | QBSBlit( |
| 0x1801B | 7 | RIGHT$( |
| 0x18025 | 4 | RND( |
| 0x18032 | 4 | ROL( |
| 0x18039 | 4 | ROR( |
| 0x18040 | 5 | ROL&( |
| 0x18048 | 5 | ROR&( |
| 0x18050 | 5 | ROL\|( |
| 0x18058 | 5 | ROR\|( |
| 0x18060 | 4 | RAD( |
| 0x18067 | 6 | ROUND( |
| 0x18070 | 7 | RINSTR( |
| 0x1807A | 8 | RasSize( |
| 0x18085 | 9 | RawDoFmt( |
| 0x18091 | 11 | RawIOInit() |
| 0x1809F | 14 | RawKeyConvert( |
| 0x180B0 | 15 | RawMayGetChar() |
| 0x180C2 | 11 | RawPutChar( |
| 0x180D0 | 7 | RANDOM( |
| 0x180DA | 5 | RAND( |
| 0x180E2 | 5 | Read( |
| 0x180EA | 10 | ReadPixel( |
| 0x180F7 | 9 | RectFill( |
| 0x18103 | 15 | RefreshGadgets( |
| 0x18115 | 13 | RefreshGList( |
| 0x18125 | 19 | RefreshWindowFrame( |
| 0x1813B | 15 | RemakeDisplay() |
| 0x1814D | 10 | RemDevice( |
| 0x1815A | 8 | RemFont( |
| 0x18165 | 8 | RemHead( |
| 0x18170 | 8 | RemIBob( |
| 0x1817B | 13 | RemIntServer( |
| 0x1818B | 11 | RemLibrary( |
| 0x18199 | 7 | Remove( |
| 0x181A3 | 13 | RemoveGadget( |
| 0x181B3 | 12 | RemoveGList( |
| 0x181C2 | 8 | RemPort( |
| 0x181CD | 12 | RemResource( |
| 0x181DC | 8 | RemTail( |
| 0x181E7 | 8 | RemTask( |
| 0x181F2 | 11 | RemVSprite( |
| 0x18200 | 7 | Rename( |
| 0x1820A | 9 | ReplyMsg( |
| 0x18216 | 12 | ReportMouse( |
| 0x18225 | 8 | Request( |
| 0x18230 | 12 | Reschedule() |
| 0x1823F | 16 | RethinkDisplay() |
| 0x18252 | 4 | SGN( |
| 0x18259 | 4 | SIN( |
| 0x18260 | 5 | SINQ( |
| 0x18268 | 7 | SPACE$( |
| 0x18272 | 4 | SPC( |
| 0x18279 | 4 | SQR( |
| 0x18280 | 4 | STEP |
| 0x18287 | 5 | STR$( |
| 0x1828F | 8 | STRING$( |
| 0x1829A | 4 | SHL( |
| 0x182A1 | 4 | SHR( |
| 0x182A8 | 5 | SHL&( |
| 0x182B0 | 5 | SHR&( |
| 0x182B8 | 5 | SHL\|( |
| 0x182C0 | 5 | SHR\|( |
| 0x182C8 | 5 | SWAP( |
| 0x182D0 | 5 | SUCC( |
| 0x182D8 | 4 | SUB( |
| 0x182DF | 6 | STICK( |
| 0x182E8 | 6 | STRIG( |
| 0x182F1 | 4 | STOP |
| 0x182F8 | 7 | SINGLE{ |
| 0x18302 | 7 | SCREEN( |
| 0x1830C | 10 | Schedule() |
| 0x18319 | 13 | ScreenToBack( |
| 0x18329 | 14 | ScreenToFront( |
| 0x1833A | 12 | ScrollLayer( |
| 0x18349 | 13 | ScrollRaster( |
| 0x18359 | 12 | ScrollVPort( |
| 0x18368 | 5 | Seek( |
| 0x18370 | 7 | SendIO( |
| 0x1837A | 8 | SetAfPt( |
| 0x18385 | 8 | SetAPen( |
| 0x18390 | 8 | SetBPen( |
| 0x1839B | 13 | SetCollision( |
| 0x183AB | 11 | SetComment( |
| 0x183B9 | 13 | SetDMRequest( |
| 0x183C9 | 8 | SetDrMd( |
| 0x183D4 | 8 | SetDrPt( |
| 0x183DF | 10 | SetExcept( |
| 0x183EC | 8 | SetFont( |
| 0x183F7 | 12 | SetFunction( |
| 0x18406 | 13 | SetIntVector( |
| 0x18416 | 13 | SetMenuStrip( |
| 0x18426 | 8 | SetOPen( |
| 0x18431 | 11 | SetPointer( |
| 0x1843F | 9 | SetPrefs( |
| 0x1844B | 14 | SetProtection( |
| 0x1845C | 8 | SetRast( |
| 0x18467 | 8 | SetRGB4( |
| 0x18472 | 10 | SetRGB4CM( |
| 0x1847F | 10 | SetSignal( |
| 0x1848C | 13 | SetSoftStyle( |
| 0x1849C | 6 | SetSR( |
| 0x184A5 | 11 | SetTaskPri( |
| 0x184B3 | 16 | SetWindowTitles( |
| 0x184C6 | 9 | SetWrMsk( |
| 0x184D2 | 10 | ShiftItem( |
| 0x184DF | 10 | ShiftMenu( |
| 0x184EC | 10 | ShiftMenu( |
| 0x184F9 | 10 | ShowTitle( |
| 0x18506 | 7 | Signal( |
| 0x18510 | 10 | SizeLayer( |
| 0x1851D | 11 | SizeWindow( |
| 0x1852B | 10 | SortGList( |
| 0x18538 | 8 | SubTime( |
| 0x18543 | 11 | SumLibrary( |
| 0x18551 | 12 | SuperState() |
| 0x18560 | 12 | Supervisor() |
| 0x1856F | 25 | SwapBitsRastPortClipRect( |
| 0x1858B | 8 | Switch() |
| 0x18596 | 12 | SyncSBitMap( |
| 0x185A5 | 4 | TAB( |
| 0x185AC | 4 | TAN( |
| 0x185B3 | 4 | THEN |
| 0x185BA | 5 | TIME$ |
| 0x185C2 | 5 | TIMER |
| 0x185CF | 4 | TRUE |
| 0x185D6 | 6 | TRUNC( |
| 0x185DF | 5 | TYPE( |
| 0x185E7 | 6 | TRACE$ |
| 0x185F0 | 6 | TRIM$( |
| 0x185F9 | 5 | Text( |
| 0x18601 | 11 | TextLength( |
| 0x1860F | 14 | ThinLayerInfo( |
| 0x18620 | 10 | Translate( |
| 0x1862D | 10 | TypeOfMem( |
| 0x1863A | 11 | TRANSLATE$( |
| 0x18648 | 7 | UPPER$( |
| 0x18652 | 5 | USING |
| 0x1865A | 16 | UCopperListInit( |
| 0x1866D | 10 | UnLoadSeg( |
| 0x1867A | 7 | UnLock( |
| 0x18684 | 12 | UnlockIBase( |
| 0x18693 | 12 | UnlockLayer( |
| 0x186A2 | 16 | UnlockLayerInfo( |
| 0x186B5 | 15 | UnlockLayerRom( |
| 0x186C7 | 13 | UnlockLayers( |
| 0x186D7 | 13 | UpfrontLayer( |
| 0x186E7 | 10 | UserState( |
| 0x186F4 | 4 | VAL( |
| 0x186FB | 5 | VAL?( |
| 0x18703 | 7 | VARIAT( |
| 0x1870D | 7 | VARPTR( |
| 0x18722 | 7 | Vacate( |
| 0x1872C | 10 | VBeamPos() |
| 0x18739 | 13 | ViewAddress() |
| 0x18749 | 16 | ViewPortAddress( |
| 0x18761 | 5 | WORD( |
| 0x18769 | 5 | WORD{ |
| 0x18771 | 4 | WITH |
| 0x18778 | 7 | WINDOW( |
| 0x18782 | 5 | Wait( |
| 0x1878A | 10 | WaitBlit() |
| 0x18797 | 9 | WaitBOVP( |
| 0x187A3 | 12 | WaitForChar( |
| 0x187B2 | 7 | WaitIO( |
| 0x187BC | 9 | WaitPort( |
| 0x187C8 | 9 | WaitTOF() |
| 0x187D4 | 14 | WBenchToBack() |
| 0x187E5 | 15 | WBenchToFront() |
| 0x187F7 | 11 | WhichLayer( |
| 0x18805 | 13 | WindowLimits( |
| 0x18815 | 13 | WindowToBack( |
| 0x18825 | 14 | WindowToFront( |
| 0x18836 | 6 | Write( |
| 0x1883F | 11 | WritePixel( |
| 0x1884D | 11 | WritePotgo( |
| 0x1885B | 14 | XorRectRegion( |
| 0x1886C | 16 | XorRegionRegion( |
| 0x18885 | 4 | XOR( |
| 0x1888C | 5 | _DATA |
| 0x188D6 | 8 | _DosBase |
| 0x188E1 | 8 | _IntBase |
| 0x188EC | 8 | _GfxBase |
| 0x188F7 | 11 | _LayersBase |
| 0x18905 | 8 | _dosCmd$ |
| 0x18910 | 9 | _IOExtSer |
| 0x1891C | 9 | _SndData( |
| 0x1893C | 5 | EOF(# |
| 0x18944 | 5 | LOF(# |
| 0x1894C | 5 | LOC(# |
| 0x18954 | 4 |  AT( |
| 0x1895B | 5 | SUCC( |
| 0x18963 | 5 | PRED( |
| 0x189A4 | 21 | Nu3333333333UUUUTDwp  |
| 0x18A87 | 4 | hNuR |
| 0x18AEE | 7 | gbkfHPa |
| 0x18B00 | 5 | g, @A |
| 0x18B12 | 5 | "@$Hp |
| 0x18B4A | 5 | Nu.n/ |
| 0x18B97 | 6 | rt@`&E |
| 0x18BB7 | 4 | C620 |
| 0x18BEA | 5 | $K L` |
| 0x18C22 | 5 | HQHPB |
| 0x18C8E | 7 | HRHQHPE |
| 0x18CA2 | 7 | HRHQHPE |
| 0x18CD8 | 4 |  W"o |
| 0x18CE4 | 4 | NuaL |
| 0x18D28 | 5 | \|@`8G |
| 0x18D38 | 5 | gz-H! |
| 0x18D57 | 4 | F<3` |
| 0x18DA8 | 5 | &M L` |
| 0x18DB0 | 8 | *HNu MNu |
| 0x18E08 | 5 | Nupb` |
| 0x18E0E | 4 | pf=@ |
| 0x18E48 | 4 | NuTn |
| 0x18F28 | 5 | JBk~N |
| 0x18F36 | 4 | dhS) |
| 0x18F60 | 10 | JBkFgHJAf$ |
| 0x19010 | 4 | NuTn |
| 0x19015 | 5 | jNu=\| |
| 0x19022 | 4 | `"=\| |
| 0x19066 | 4 | Nu3\| |
| 0x19078 | 5 | NuADD |
| 0x19086 | 4 | BASE |
| 0x1908C | 4 | 1CPY |
| 0x1909E | 5 | uQDET |
| 0x190AC | 5 | INPUT |
| 0x190B9 | 5 | 2NORM |
| 0x190CD | 5 | PRINT |
| 0x190D9 | 4 | READ |
| 0x190E0 | 4 | RANG |
| 0x190F2 | 6 | %TRANS |
| 0x190FB | 4 | XCPY |
| 0x19146 | 4 | "ANu |
| 0x19A3E | 5 | b!) F |
| 0x19FBF | 5 | Z\^b` |
| 0x1A034 | 4 | BBBB |
| 0x1A0C7 | 4 | @@@T |
| 0x1A193 | 4 | x$6$ |
| 0x1A736 | 4 |  _Nu |
| 0x1A818 | 5 | f,J./ |
| 0x1A96E | 4 | "HNu |
| 0x1AA76 | 5 | Jn/8g |
| 0x1AAEA | 42 | `.# $ % ! #($(%(!(& \ :   &(\(  $ Ok\|Error |
| 0x1AB21 | 4 | hp?  |
| 0x1AB3E | 4 | "F : |
| 0x1AB72 | 4 |  F"n |
| 0x1AB82 | 4 | "F : |
| 0x1AB8D | 6 | ~N[SEf |
| 0x1AB9E | 4 | SATA |
| 0x1ABBC | 4 | @dB0 |
| 0x1ABE8 | 7 | @dJ./=f |
| 0x1ABFD | 4 | R(HK |
| 0x1AC0C | 5 | Nu(IK |
| 0x1AC62 | 5 | NuB./ |
| 0x1AC89 | 4 | &gZr |
| 0x1ACD6 | 7 | p?NuJ./ |
| 0x1ACED | 4 | XgRr |
| 0x1ACF5 | 4 | Og0r |
| 0x1ADCE | 7 |  _NuHPa |
| 0x1AE3B | 6 | >g@Nur |
| 0x1AE9F | 6 | F"HNuB |
| 0x1AECB | 4 | EfBR |
| 0x1AEE2 | 4 | `,Nu |
| 0x1AF8E | 5 | "HNu" |
| 0x1B209 | 4 | =/ G |
| 0x1B2FA | 5 | g0b<` |
| 0x1B3B7 | 4 | ./ g |
| 0x1B3BD | 5 | \B./  |
| 0x1B3D0 | 9 | #($(%(!(& |
| 0x1B3E0 | 4 | &(\|( |
| 0x1B4F8 | 5 | k g U |
| 0x1B4FE | 5 | k.g.U |
| 0x1B60A | 5 | k(g<U |
| 0x1B610 | 4 | k(g. |
| 0x1B631 | 4 | "Nu  |
| 0x1B6F8 | 5 | $,$HL |
| 0x1B777 | 4 | =/ r |
| 0x1B7B3 | 4 | ,/ ` |
| 0x1B7C5 | 4 | (/ p |
| 0x1B816 | 5 | 'NJBj |
| 0x1B821 | 4 | FZBN |
| 0x1B85C | 5 | DBNu$ |
| 0x1B894 | 4 | NuSB |
| 0x1B9C0 | 5 | 8Mzh1 |
| 0x1B9E0 | 4 | .2~l |
| 0x1BA10 | 5 | &IUGj |
| 0x1BA1E | 5 | NuSGk |
| 0x1BAA9 | 4 | 8$L/ |
| 0x1BB01 | 6 | ,&.$Xg |
| 0x1BB0B | 4 | 0$\a |
| 0x1BB15 | 4 | 8$4a |
| 0x1BB20 | 4 | $4 < |
| 0x1BB36 | 5 | &.$dg |
| 0x1BB3F | 4 | 0$ha |
| 0x1BB49 | 4 | 8$@a |
| 0x1BB5A | 5 | J.$pf |
| 0x1BBCD | 6 | 8$4adH |
| 0x1BBE3 | 6 | 8$@aNH |
| 0x1BC1B | 6 | bazJBf |
| 0x1BC5C | 5 | NuJBk |
| 0x1BC6E | 5 | NuSB` |
| 0x1BCB2 | 5 | RG K  |
| 0x1BCCB | 4 | HNuN |
| 0x1BCE9 | 4 | .$pf |
| 0x1BD1C | 5 | $LNuA |
| 0x1BDC1 | 4 | rfdR |
| 0x1BE76 | 6 | $@zlxd |
| 0x1BE82 | 7 | zoxc&_" |
| 0x1BEDC | 5 | "{p8, |
| 0x1BFED | 4 | xNup |
| 0x1BFF9 | 4 | lNu, |
| 0x1C103 | 4 | bSGk |
| 0x1C113 | 4 | FNuT |
| 0x1C159 | 4 | bSGk |
| 0x1C27C | 5 | o">3X |
| 0x1C29D | 8 | 0Nug">3X |
| 0x1C2C1 | 6 | 0Nu>3X |
| 0x1C306 | 5 | NupLN |
| 0x1C334 | 5 | kL WN |
| 0x1C34A | 4 | `2.: |
| 0x1C3CC | 4 | RH=H |
| 0x1C3E3 | 4 | hpdN |
| 0x1C428 | 5 | S@f(. |
| 0x1C45C | 4 | `..: |
| 0x1C482 | 4 | p,a\ |
| 0x1C4C5 | 4 | &"HN |
| 0x1C559 | 4 | \|U@f |
| 0x1C5AA | 4 | 0@2A |
| 0x1C5C6 | 4 | 2@ < |
| 0x1C5F6 | 5 | b RB` |
| 0x1C622 | 6 | fTJ@kP |
| 0x1C632 | 4 | 0@ < |
| 0x1C66D | 6 | NSISH6 |
| 0x1C676 | 5 | NupVN |
| 0x1C6B6 | 5 | 'NHA2 |
| 0x1C6BF | 4 | 8UCf |
| 0x1C750 | 5 | S@fb/ |
| 0x1C923 | 5 | \NuHy |
| 0x1C973 | 4 | Wfda |
| 0x1C97B | 5 | _f\-I |
| 0x1C99F | 5 | Bf8=z |
| 0x1C9D8 | 5 | NupMN |
| 0x1C9F4 | 4 | gH8. |
| 0x1CA04 | 4 | `8=z |
| 0x1CA8E | 4 | f2=z |
| 0x1CAC0 | 4 | Nu=z |
| 0x1CAC8 | 4 | g@8. |
| 0x1CAD4 | 4 | `4=z |
| 0x1CBF2 | 4 |  W"o |
| 0x1CC02 | 4 | `n0/ |
| 0x1CC0F | 5 | Acf0/ |
| 0x1CC1D | 4 | AcX~ |
| 0x1CC22 | 4 | `F2/ |
| 0x1CC2F | 4 | Cc>H |
| 0x1CC3F | 4 | Dc.H |
| 0x1CC7B | 5 | "Nu0/ |
| 0x1CC89 | 5 | Acr0/ |
| 0x1CC97 | 4 | Acd~ |
| 0x1CC9C | 4 | `N2/ |
| 0x1CCA9 | 5 | CcH0/ |
| 0x1CCB7 | 4 | Dc:H |
| 0x1CD01 | 6 | "NupMN |
| 0x1CD73 | 8 | 0SFSESDg |
| 0x1CD7E | 8 | JEf~JFfz |
| 0x1CDFF | 4 | TJDf |
| 0x1CE30 | 4 | gZSo |
| 0x1CE42 | 4 | `HBo |
| 0x1CEBD | 4 | .S@H |
| 0x1CF87 | 4 | &S@H |
| 0x1D055 | 6 | RNupTN |
| 0x1D07F | 4 | VS@f |
| 0x1D091 | 4 | DS@f |
| 0x1D0DF | 4 | R _  |
| 0x1D0E8 | 5 | NupON |
| 0x1D13D | 4 |  S@f |
| 0x1D167 | 4 | 8`BH |
| 0x1D1FD | 4 | R _  |
| 0x1D241 | 4 | $U@f |
| 0x1D285 | 6 | <$HSB2 |
| 0x1D2B9 | 5 | "RB?B |
| 0x1D31C | 4 | HA>/ |
| 0x1D331 | 4 | H`4& |
| 0x1D39F | 5 | CgF0/ |
| 0x1D3EF | 5 | HgH?o |
| 0x1D48E | 4 | `v8/ |
| 0x1D4DC | 4 | DBSB |
| 0x1D548 | 4 | g>0/ |
| 0x1D559 | 4 | Tn@  |
| 0x1D58B | 4 | P _  |
| 0x1D596 | 5 | NupRN |
| 0x1D5AD | 4 | 8SAn |
| 0x1D5DF | 4 | 8DBN |
| 0x1D715 | 6 | PNu JC |
| 0x1D75C | 4 | DB#@ |
| 0x1D773 | 4 | \|DB$ |
| 0x1D791 | 6 | PNu JC |
| 0x1D7F9 | 4 | 8DBa |
| 0x1D825 | 4 | 8DEa |
| 0x1D853 | 4 | 8DBa |
| 0x1D889 | 4 | 8DEa |
| 0x1D8B5 | 4 | 8DBa |
| 0x1D8E1 | 4 | 8DEa |
| 0x1D917 | 4 | 8DBa |
| 0x1D943 | 4 | 8DEa |
| 0x1D96F | 4 | 8DBa |
| 0x1D979 | 4 | PNuO |
| 0x1D98B | 4 |  U@f |
| 0x1D9CE | 5 | $HSB2 |
| 0x1D9F5 | 5 | "RB?B |
| 0x1DA05 | 4 | 8 Wa |
| 0x1DA53 | 4 | BJp  |
| 0x1DA86 | 4 | HAHC |
| 0x1DAE3 | 4 | Jg*  |
| 0x1DAF5 | 4 | NDBH |
| 0x1DB4B | 4 | 2ghH |
| 0x1DB59 | 4 | < 2H |
| 0x1DB75 | 4 | Jg,H |
| 0x1DBCF | 5 | @g.>/ |
| 0x1DBDD | 4 | < 2x |
| 0x1DC1E | 4 | g^0/ |
| 0x1DC55 | 4 | E:0P |
| 0x1DC5F | 4 | D80@ |
| 0x1DC81 | 6 | PNupVN |
| 0x1DC8C | 12 | Select fehlt |
| 0x1DC99 | 15 | Endselect fehlt |
| 0x1DCAB | 18 | ll - Editor Fehler |
| 0x1DCBE | 10 | Wend fehlt |
| 0x1DCC9 | 11 | Until fehlt |
| 0x1DCD5 | 10 | Loop fehlt |
| 0x1DCE0 | 10 | Next fehlt |
| 0x1DCEB | 11 | While fehlt |
| 0x1DCF7 | 12 | Repeat fehlt |
| 0x1DD04 | 8 | Do fehlt |
| 0x1DD0D | 9 | For fehlt |
| 0x1DD17 | 11 | Endif fehlt |
| 0x1DD23 | 8 | If fehlt |
| 0x1DD2C | 11 | Endif fehlt |
| 0x1DD38 | 18 | Exit ohne Schleife |
| 0x1DD4B | 12 | Return fehlt |
| 0x1DD58 | 21 | Procedure in Schleife |
| 0x1DD6E | 17 | Procedure doppelt |
| 0x1DD80 | 13 | Endfunc fehlt |
| 0x1DD8E | 20 | Function in Schleife |
| 0x1DDA3 | 16 | Function doppelt |
| 0x1DDB4 | 15 | Procedure fehlt |
| 0x1DDC4 | 13 | Marke doppelt |
| 0x1DDD2 | 34 | Local ohne Procedure oder Function |
| 0x1DDF5 | 17 | Local in Schleife |
| 0x1DE07 | 35 | Goto in/aus For-Next oder Procedure |
| 0x1DE2B | 27 | Resume in For-Next-Schleife |
| 0x1DE47 | 21 | Resume ohne Procedure |
| 0x1DE5D | 23 | Kein Resume in Function |
| 0x1DE75 | 14 | Function fehlt |
| 0x1DE84 | 13 | Neue Variable |
| 0x1DE92 | 14 | Neue Procedure |
| 0x1DEA1 | 13 | Neue Function |
| 0x1DEAF | 11 | Neues Label |
| 0x1DEBB | 4 | Best |
| 0x1DEC0 | 32 | tigung bei neuen\|Variablennamen? |
| 0x1DEE1 | 7 | Ja\|Nein |
| 0x1DEE9 | 12 |   Neue Namen |
| 0x1DEF6 | 8 | Inline l |
| 0x1DEFF | 7 | schen ? |
| 0x1DF07 | 8 | Ok\|Error |
| 0x1DF11 | 19 | Division durch Null |
| 0x1DF27 | 7 | berlauf |
| 0x1DF30 | 44 | Zahl nicht Integer\|-2147483648 .. 2147483647 |
| 0x1DF5E | 24 | Zahl nicht Byte\|0 .. 255 |
| 0x1DF78 | 31 | Zahl nicht Wort\|-32768 .. 32767 |
| 0x1DF99 | 19 | Quadratwurzel nur f |
| 0x1DFAD | 17 | r\|positive Zahlen |
| 0x1DFC0 | 17 | Logarithmen nur f |
| 0x1DFD2 | 11 | r\|Zahlen gr |
| 0x1DFDF | 7 | er Null |
| 0x1DFE8 | 19 | Unbekannter Fehler  |
| 0x1DFFD | 13 | Speicher voll |
| 0x1E00C | 33 | Funktion oder Befehl\|noch nicht m |
| 0x1E02E | 5 | glich |
| 0x1E035 | 33 | String zu lang\|max. 32767 Zeichen |
| 0x1E058 | 28 | Kein GFA-BASIC-3.52-Programm |
| 0x1E076 | 34 | Programm zu lang\|Speicher voll\|NEW |
| 0x1E09A | 33 | Kein GFA-BASIC-Programm\|EOF - NEW |
| 0x1E0BD | 26 | Feld zweimal dimensioniert |
| 0x1E0D9 | 24 | Feld nicht dimensioniert |
| 0x1E0F3 | 16 | Feldindex zu gro |
| 0x1E106 | 10 | Dim zu gro |
| 0x1E113 | 22 | Falsche Anzahl Indizes |
| 0x1E12B | 24 | Procedure nicht gefunden |
| 0x1E145 | 20 | Label nicht gefunden |
| 0x1E15B | 66 | Bei Open nur erlaubt:\|"I"nput  "O"utput "R"andom\|"A"ppend "U"pdate |
| 0x1E19F | 13 | File schon ge |
| 0x1E1AD | 5 | ffnet |
| 0x1E1B4 | 13 | File # falsch |
| 0x1E1C3 | 13 | File nicht ge |
| 0x1E1D1 | 5 | ffnet |
| 0x1E1D8 | 27 | Falsche Eingabe, keine Zahl |
| 0x1E1F5 | 21 | Fileende erreicht\|EOF |
| 0x1E20C | 17 | Zu viele Punkte f |
| 0x1E21E | 31 | r\|Polyline/Polyfill\|maximal 128 |
| 0x1E23F | 7 | Feld mu |
| 0x1E247 | 20 |  eindimensional sein |
| 0x1E25D | 16 | Anzahl Punkte gr |
| 0x1E26F | 11 | er als Feld |
| 0x1E27C | 23 | Merge - Kein ASCII-File |
| 0x1E295 | 31 | Merge - Zeile zu lang - Abbruch |
| 0x1E2B5 | 41 |  ==> Syntax nicht korrekt\|Programmabbruch |
| 0x1E2DF | 22 | !Marke nicht definiert |
| 0x1E2F6 | 14 | "Zu wenig Data |
| 0x1E305 | 21 | #Data nicht numerisch |
| 0x1E31B | 14 | %Diskette voll |
| 0x1E32A | 32 | &Befehl im Direktmodus \| nicht m |
| 0x1E34B | 5 | glich |
| 0x1E351 | 30 | 'Programmfehler \| Kein Gosub m |
| 0x1E370 | 5 | glich |
| 0x1E376 | 14 | (Clear nicht m |
| 0x1E385 | 43 | glich in\|For-Next-Schleifen oder\|Proceduren |
| 0x1E3B1 | 13 | )Cont nicht m |
| 0x1E3BF | 5 | glich |
| 0x1E3C5 | 19 | *Zu wenig Parameter |
| 0x1E3D9 | 20 | +Ausdruck zu komplex |
| 0x1E3EE | 25 | ,Funktion nicht definiert |
| 0x1E408 | 19 | -Zu viele Parameter |
| 0x1E41C | 28 | .Parameter falsch\|keine Zahl |
| 0x1E439 | 29 | /Parameter falsch\|kein String |
| 0x1E457 | 17 | 0Open "R" - Satzl |
| 0x1E469 | 10 | nge falsch |
| 0x1E474 | 29 | 1Zu viele "R"-Files (max. 31) |
| 0x1E492 | 14 | 2Kein "R"-File |
| 0x1E4A1 | 10 | 4Fields gr |
| 0x1E4AD | 12 | er als Satzl |
| 0x1E4BE | 23 | 6GET/PUT Field-String\|L |
| 0x1E4D6 | 10 | nge falsch |
| 0x1E4E1 | 26 | 7GET/PUT Satznummer falsch |
| 0x1E4FC | 16 | <Sprite-String-L |
| 0x1E50D | 10 | nge falsch |
| 0x1E518 | 19 | =Fehler bei RESERVE |
| 0x1E52C | 12 | >Menu falsch |
| 0x1E539 | 15 | ?Reserve falsch |
| 0x1E549 | 15 | @Pointer falsch |
| 0x1E559 | 7 | AFeldgr |
| 0x1E562 | 7 | e < 256 |
| 0x1E56A | 15 | BKein VAR-Array |
| 0x1E57A | 17 | CASIN/ACOS falsch |
| 0x1E58C | 17 | DFalsche VAR-Type |
| 0x1E59E | 20 | EENDFUNC ohne RETURN |
| 0x1E5B3 | 23 | GIndex/Parameter zu gro |
| 0x1E5CC | 27 | IKann Fenster/Screen nicht  |
| 0x1E5E8 | 5 | ffnen |
| 0x1E5EE | 15 | HZu viele Eintr |
| 0x1E601 | 14 | JFenster fehlt |
| 0x1E610 | 17 | KLibrary nicht ge |
| 0x1E622 | 5 | ffnet |
| 0x1E628 | 26 | LMatrizenoperationen nur f |
| 0x1E643 | 35 | r\|ein- oder zweidimensionale\|Felder |
| 0x1E667 | 41 | MMatrizen haben nicht die\|gleiche Ordnung |
| 0x1E691 | 30 | NVektorprodukt nicht definiert |
| 0x1E6B0 | 32 | OMatrizenprodukt nicht definiert |
| 0x1E6D1 | 30 | PScalarprodukt nicht definiert |
| 0x1E6F0 | 20 | QTransposition nur f |
| 0x1E705 | 27 | r\|zweidimensionale Matrizen |
| 0x1E721 | 25 | RMatrix nicht quadratisch |
| 0x1E73B | 30 | STransposition nicht definiert |
| 0x1E75A | 35 | VFACT/COMBIN/PERMUT\|nicht definiert |
| 0x1E77E | 35 | XSpeicheranforderung fehlgeschlagen |
| 0x1E7A2 | 15 | TObject. Fehler |
| 0x1E7B2 | 15 | UDrucker Fehler |
| 0x1E7C2 | 14 | YSprite Fehler |
| 0x1E7D1 | 17 | ZFehler bei Local |
| 0x1E7E3 | 15 | [Fehler bei For |
| 0x1E7F3 | 22 | \Resume (next) nicht m |
| 0x1E80A | 27 | glich\|Fatal, For oder Local |
| 0x1E826 | 14 | ]Stapel-Fehler |
| 0x1E835 | 26 | dGFA-BASIC Version 3.52 D\| |
| 0x1E850 | 43 |  Copyright 1986-1990\|GFA Systemtechnik GmbH |
| 0x1E87C | 24 | gUnzureichender Speicher |
| 0x1E895 | 18 | hTask-Tabelle voll |
| 0x1E8A8 | 16 | mGuru Meditation |
| 0x1E8B9 | 10 | nBus Error |
| 0x1E8C4 | 14 | oAddress Error |
| 0x1E8D3 | 20 | pIllegal Instruction |
| 0x1E8E8 | 12 | qZero Divide |
| 0x1E8F5 | 16 | rCHK Instruction |
| 0x1E906 | 18 | sTRAPV Instruction |
| 0x1E919 | 20 | tPrivilege Violation |
| 0x1E92E | 6 | uTrace |
| 0x1E935 | 16 | vLine A Emulator |
| 0x1E946 | 16 | wLine F Emulator |
| 0x1E957 | 19 | xArgument-Zeile ung |
| 0x1E96B | 17 | ltig\|oder zu lang |
| 0x1E97D | 10 | yKein ausf |
| 0x1E988 | 16 | hrbares Programm |
| 0x1E999 | 4 | zUng |
| 0x1E99E | 47 | ltige residente Library\|beim Laden angesprochen |
| 0x1E9CF | 30 | Objekt wird schon benutzt\|Lock |
| 0x1E9EF | 22 | Objekt existiert schon |
| 0x1EA07 | 26 | Verzeichnis nicht gefunden |
| 0x1EA23 | 21 | Objekt nicht gefunden |
| 0x1EA3E | 14 | ltiges Fenster |
| 0x1EA4E | 21 | Unerlaubter Dateiname |
| 0x1EA65 | 22 | Objekt-Typ passt nicht |
| 0x1EA7D | 22 | Diskette NOT VALIDATED |
| 0x1EA95 | 21 | Diskette schreibgesch |
| 0x1EAB0 | 28 | RENAME mit verschiedenen Ger |
| 0x1EAD2 | 22 | Verzeichnis nicht leer |
| 0x1EAEE | 14 | t nicht bereit |
| 0x1EAFE | 30 | Datei-Kommentar zu umfangreich |
| 0x1EB1E | 13 | Diskette voll |
| 0x1EB2D | 17 | Datei ist gegen L |
| 0x1EB3F | 11 | schen gesch |
| 0x1EB50 | 22 | Datei ist schreibgesch |
| 0x1EB6C | 19 | Datei ist lesegesch |
| 0x1EB85 | 18 | Keine DOS Diskette |
| 0x1EB99 | 26 | Keine Diskette im Laufwerk |
| 0x1EBB5 | 31 | Unbekannter PACKET REQUEST TYPE |
| 0x1EBD6 | 23 | Unbekannter OBJECT LOCK |
| 0x1EBEF | 11 | SEEK Fehler |
| 0x1EBFC | 32 | Keine weiteren Verzeichnis-Eintr |
| 0x1EC21 | 46 | Wollen Sie den Interpreter\|wirklich verlassen? |
| 0x1EC50 | 9 |  Ja \|Nein |
| 0x1EC5A | 12 | Programmende |
| 0x1EC67 | 13 | Programmstop? |
| 0x1EC75 | 9 | Stop\|Cont |
| 0x1EC7F | 15 | Druckerlisting? |
| 0x1EC8F | 7 | Ja\|Nein |
| 0x1EC97 | 16 | NEW - Programm L |
| 0x1ECA8 | 6 | schen? |
| 0x1ECAF | 7 | Ja\|Nein |
| 0x1ECBA | 13 | Syntax Fehler |
| 0x1ECCC | 13 | Zeile zu lang |
| 0x1ECDB | 7 | Abbruch |
| 0x2149C | 10 | GFA-AMIGAB |
| 0x214BE | 16 | GFA-BASIC-Screen |
| 0x214D4 | 7 | GFABobs |
| 0x214EA | 10 | NormaInter |
| 0x2151C | 6 | Parent |
| 0x21523 | 6 | GetDir |

## String cross-references

`string_xrefs.jsonl` maps each extracted file-offset string to Ghidra references targeting its imported address. It records source addresses and containing functions where Ghidra identified one. This is a useful bridge from user-visible diagnostics/keywords to code, but a string reference alone does not prove that the string is a BASIC token or that the referencing function implements it.

| String | File offset | Referencing function(s) | Source address(es) |
|---|---:|---|---|
| ,@-\| | 0xE6 | (no containing function) | 0x1473E |
| J4H\N | 0x11E | FUN_0001cb16 | 0x1CBB0 |
| SBDBj | 0x53A | FUN_0000043e | 0x50A |
| DBNug | 0x5C2 | (no containing function), FUN_00000914 | 0x8AE, 0x8C8, 0x94A, 0x966 |
| DBNut | 0x5E2 | FUN_000087e2 | 0x87EA |
| DEDBH | 0x68C | (no containing function) | 0x15B6E |
| DBv- | 0x924 | (no containing function), FUN_00000914, FUN_00007d58 | 0xBFE, 0x8C0, 0x944 |
| H@DBNuH@RBDBNu4< | 0xC76 | FUN_00000c28 | 0xC34 |
| SBDB | 0x10EA | FUN_000014f6 | 0x1502, 0x1570 |
| RBDBH | 0x1806 | (no containing function) | 0x1B600 |
| JDgH2 | 0x1A10 | FUN_00000676 | 0x19D8, 0x19DC, 0x19E0 |
| HCH@ | 0x1A1C | FUN_00000676 | 0x1974 |
| GFA-BASIC Commands | 0x1B9E | FUN_00001cbc | 0x1D38 |
| X@Nua | 0x3696 | (no containing function) | 0x3668 |
|  _Nua | 0x36B8 | (no containing function) | 0x368C |
| &@ S` | 0x468C | FUN_0000461c | 0x4632 |
| &@0+ | 0x469E | FUN_0000408c, FUN_0000464e, FUN_0000a0be | 0x4664, 0xA0CC, 0x40A0 |
| J@g2 | 0x46E8 | FUN_00004676 | 0x46BA |
|  W P | 0x492A | (no containing function) | 0x48F8, 0x4906 |
|  W4( | 0x49EC | FUN_000049a2 | 0x49A8, 0x49AC, 0x49B2 |
| DBNu | 0x4C36 | (no containing function) | 0x4C0A |
|  @4( | 0x4D0A | (no containing function) | 0x7050 |
| Nu? P | 0x52A2 | (no containing function) | 0x527E, 0x5282, 0x5294 |
| BgHUa | 0x52BC | (no containing function) | 0x24DC, 0x24E0 |
| SG`4 | 0x5890 | FUN_00005754 | 0x5858 |
| SGSF | 0x5976 | FUN_00005754 | 0x593C |
| -@/$ @"X0 | 0x5CB0 | FUN_000069aa | 0x5CCC |
| /(Rn/(Sn/*kz | 0x5CEE | FUN_000069aa | 0x5D3A, 0x5FC6, 0x612A |
| Sn/*k | 0x5E14 | FUN_000069aa | 0x5DFE |
| =n/6/4j | 0x5ED8 | FUN_000069aa | 0x5E50, 0x5E56, 0x5E7A, 0x5E80, 0x5EA8 |
| /(Rn/(0 | 0x5F30 | FUN_000069aa | 0x5EFC |
| Jn/4j | 0x5FA6 | FUN_000069aa | 0x5F44, 0x5F64, 0x5F6C |
| =_/(Sn/(Rn/*` | 0x5FBA | FUN_000069aa | 0x5F50 |
| g4 A | 0x6676 | (no containing function) | 0x6612, 0x661C, 0x6626 |
| J_Nu | 0x6838 | (no containing function), FUN_000069aa | 0x9016, 0x5D6E |
| UP-@i | 0x6B8C | FUN_00006ac0 | 0x6ADA, 0x6AF8, 0x6B0A, 0x6B2A, 0x6B4A |
| HP Sz | 0x70DE | (no containing function) | 0x70B8 |
| afHPp | 0x7148 | (no containing function) | 0x2334, 0x2338, 0x713A |
| Nu2( | 0x7554 | FUN_00007bfe | 0x7C20, 0x7C3A |
| Nu n | 0x805C | (no containing function) | 0x8044, 0x804E |
| g:" k | 0x8154 | FUN_0000811c | 0x814E |
| gz$6p` | 0x81FE | (no containing function) | 0x81CC |
| gBnVa | 0x82A2 | (no containing function) | 0x81FE |
| "_*n | 0x8FA6 | (no containing function) | 0x8F76 |
|  <0000 | 0x9496 | FUN_0000939e | 0x93F8 |
|  Gr 0. | 0x954E | FUN_000094a6 | 0x951C |
|  n. 0 | 0x9B2C | (no containing function) | 0x9AFA |
| SPNu0 | 0x9E14 | FUN_00009de4 | 0x9DF2 |
| n (]Nup | 0xA07E | FUN_0000a04e | 0xA05C, 0xA060, 0xA064 |
| a: j | 0xA2EE | (no containing function) | 0x1F68 |
| NuJEj | 0xA394 | (no containing function) | 0xA464 |
| :!8!&!JEk | 0xA3BE | (no containing function) | 0xA37A |
| NuJBj | 0xA3E0 | (no containing function) | 0xA468 |
| NuHz | 0xAA58 | FUN_0000a9f8 | 0xAA3E |
| "_HPN | 0xAA62 | FUN_0000a9f8 | 0xAA06 |
| &@"@A | 0xB152 | (no containing function) | 0xB08C |
| (_B. | 0xC45C | (no containing function) | 0xC42E |
| NupGN | 0xCD6C | (no containing function) | 0xCD40 |
| g "n' | 0xCFB6 | FUN_0000cf04 | 0xCF88 |
|  WaNa | 0xD102 | FUN_0000d0c8 | 0xD0CC |
|  DHz | 0xD6E2 | FUN_0000d6a2 | 0xD6BC |
| $_&_2 | 0xD808 | (no containing function) | 0xD7C4, 0xD7CC, 0xD7D4 |
| Nu$n | 0xDF20 | FUN_0000de14 | 0xDEEA |
|  F$X0 | 0xE4B2 | (no containing function) | 0xE48C |
| Nu 9 | 0xE58A | FUN_0000ec78 | 0xECC8 |
| Nu:.g | 0xE618 | (no containing function) | 0xE5EC |
| HQJ) | 0xE8D8 | FUN_0000e884 | 0xE8A4 |
| hD(< | 0xE930 | FUN_0000e8b8 | 0xE8DE |
| R.h> | 0xE9F2 | FUN_0000e958 | 0xE974 |
| h2BZ4 | 0xEB56 | FUN_0000e958 | 0xE9D8, 0xEA88 |
| iZ,00 | 0xF12A | (no containing function) | 0xF08E |
| "_$IJ./ | 0xF5C0 | (no containing function) | 0xF592 |
| $I=F | 0x10172 | FUN_00010132 | 0x10146 |
| YBTB=B | 0x1051E | FUN_000104b6 | 0x104DE, 0x104E6 |
|  @"ANuN | 0x1240C | (no containing function), FUN_00012b82 | 0x12B82, 0x12CF2 |
| ,GNua | 0x1244C | (no containing function), FUN_0001289e | 0x1289E, 0x128C2 |
| ,GNua | 0x12D90 | (no containing function) | 0x12D58 |
| ,@Bn | 0x14622 | FUN_000141dc, FUN_00014218, FUN_00014418, FUN_000145bc | 0x144B6, 0x1420E, 0x143F0, 0x145E8 |
| .fpK2 | 0x149D0 | FUN_00014998 | 0x149AA |
| GFA-BASIC | 0x152AC | FUN_0000cf04 | 0xCF7A |
| r0!A | 0x15592 | FUN_00015582 | 0x1559E |
| "Jp"` | 0x155B0 | FUN_00015582 | 0x155A4 |
| `J"y | 0x1561E | FUN_0001532c | 0x15338 |
| gd&@a | 0x15DD4 | FUN_00015da2 | 0x15DA6 |
| NuaL | 0x18CE4 | (no containing function) | 0x18C4E, 0x18CD0, 0x18CD6 |
| $,$HL | 0x1B6F8 | (no containing function) | 0x1B6D2 |
| kL WN | 0x1C334 | (no containing function), FUN_0001c7b6 | 0x1C7D2, 0x1C7E2, 0x1C826, 0x1C874, 0x1C886, 0x1C8D4, 0x1C692, 0x1C696, 0x1C45E, 0x1C756 |
| Do fehlt | 0x1DD04 | (no containing function) | 0x13306 |
| For fehlt | 0x1DD0D | (no containing function) | 0x133CC |
| If fehlt | 0x1DD23 | (no containing function) | 0x13452 |
| Exit ohne Schleife | 0x1DD38 | (no containing function) | 0x1355E |
| Procedure doppelt | 0x1DD6E | (no containing function) | 0x1362E |
| Function in Schleife | 0x1DD8E | (no containing function) | 0x13756 |
| Function doppelt | 0x1DDA3 | (no containing function) | 0x13760 |
| Local ohne Procedure oder Function | 0x1DDD2 | (no containing function) | 0x136D8 |
| Local in Schleife | 0x1DDF5 | (no containing function) | 0x136CE |
| Resume ohne Procedure | 0x1DE47 | (no containing function) | 0x137A8 |
| Division durch Null | 0x1DF11 | FUN_0000889c | 0x88AC |
| Unbekannter Fehler  | 0x1DFE8 | FUN_0000889c | 0x88B4, 0x88BE |
| Wollen Sie den Interpreter\|wirklich verlassen? | 0x1EC21 | (no containing function) | 0xB2F0 |
| Syntax Fehler | 0x1ECBA | FUN_0000fd90 | 0xFF66 |
| Zeile zu lang | 0x1ECCC | FUN_0000fd90 | 0xFF5E |

## Instruction and reference indexes

`instructions.jsonl` contains decoded instructions from executable memory blocks; `references.jsonl` records Ghidra references originating at those instructions. Addresses are Ghidra address-space offsets, not guaranteed file offsets.

## Instructions outside known functions

unclaimed_instructions.jsonl contains decoded executable instructions for which Ghidra did not assign a containing function. These are candidates for missed function boundaries, not proof of a single contiguous routine.

## Function inventory

| Entry | Name | Size (bytes) | Decompilation |
|---:|---|---:|---|
| 0x406 | `FUN_00000406` | 4 | `function_00000406_FUN_00000406.c` |
| 0x416 | `FUN_00000416` | 20 | `function_00000416_FUN_00000416.c` |
| 0x43E | `FUN_0000043e` | 234 | `function_0000043E_FUN_0000043e.c` |
| 0x554 | `FUN_00000554` | 46 | `function_00000554_FUN_00000554.c` |
| 0x59A | `FUN_0000059a` | 32 | `function_0000059A_FUN_0000059a.c` |
| 0x5C2 | `FUN_000005c2` | 4 | `function_000005C2_FUN_000005c2.c` |
| 0x5EA | `FUN_000005ea` | 68 | `function_000005EA_FUN_000005ea.c` |
| 0x5F2 | `FUN_000005f2` | 97 | `function_000005F2_FUN_000005f2.c` |
| 0x676 | `FUN_00000676` | 442 | `function_00000676_FUN_00000676.c` |
| 0x790 | `FUN_00000790` | 10 | `function_00000790_FUN_00000790.c` |
| 0x79A | `FUN_0000079a` | 30 | `function_0000079A_FUN_0000079a.c` |
| 0x8EC | `FUN_000008ec` | 1 | `function_000008EC_FUN_000008ec.c` |
| 0x914 | `FUN_00000914` | 250 | `function_00000914_FUN_00000914.c` |
| 0xA1A | `FUN_00000a1a` | 10 | `function_00000A1A_FUN_00000a1a.c` |
| 0xA26 | `FUN_00000a26` | 20 | `function_00000A26_FUN_00000a26.c` |
| 0xB06 | `FUN_00000b06` | 54 | `function_00000B06_FUN_00000b06.c` |
| 0xB12 | `FUN_00000b12` | 38 | `function_00000B12_FUN_00000b12.c` |
| 0xB6C | `FUN_00000b6c` | 44 | `function_00000B6C_FUN_00000b6c.c` |
| 0xB9C | `FUN_00000b9c` | 6 | `function_00000B9C_FUN_00000b9c.c` |
| 0xBA2 | `FUN_00000ba2` | 60 | `function_00000BA2_FUN_00000ba2.c` |
| 0xC28 | `FUN_00000c28` | 100 | `function_00000C28_FUN_00000c28.c` |
| 0xC70 | `FUN_00000c70` | 12 | `function_00000C70_FUN_00000c70.c` |
| 0xC98 | `FUN_00000c98` | 62 | `function_00000C98_FUN_00000c98.c` |
| 0xED6 | `FUN_00000ed6` | 32 | `function_00000ED6_FUN_00000ed6.c` |
| 0xF00 | `FUN_00000f00` | 46 | `function_00000F00_FUN_00000f00.c` |
| 0xF50 | `FUN_00000f50` | 8 | `function_00000F50_FUN_00000f50.c` |
| 0x10C2 | `FUN_000010c2` | 296 | `function_000010C2_FUN_000010c2.c` |
| 0x11E4 | `thunk_FUN_0000043e` | 4 | `function_000011E4_thunk_FUN_0000043e.c` |
| 0x14F6 | `FUN_000014f6` | 98 | `function_000014F6_FUN_000014f6.c` |
| 0x1598 | `FUN_00001598` | 6 | `function_00001598_FUN_00001598.c` |
| 0x15AE | `FUN_000015ae` | 18 | `function_000015AE_FUN_000015ae.c` |
| 0x1610 | `FUN_00001610` | 310 | `function_00001610_FUN_00001610.c` |
| 0x1AB0 | `FUN_00001ab0` | 8 | `function_00001AB0_FUN_00001ab0.c` |
| 0x1AD8 | `FUN_00001ad8` | 8 | `function_00001AD8_FUN_00001ad8.c` |
| 0x1AE0 | `FUN_00001ae0` | 86 | `function_00001AE0_FUN_00001ae0.c` |
| 0x1CBC | `FUN_00001cbc` | 214 | `function_00001CBC_FUN_00001cbc.c` |
| 0x1D78 | `thunk_FUN_0000ceb0` | 6 | `function_00001D78_thunk_FUN_0000ceb0.c` |
| 0x1D7E | `FUN_00001d7e` | 220 | `function_00001D7E_FUN_00001d7e.c` |
| 0x1E76 | `FUN_00001e76` | 44 | `function_00001E76_FUN_00001e76.c` |
| 0x1ED8 | `FUN_00001ed8` | 20 | `function_00001ED8_FUN_00001ed8.c` |
| 0x274E | `FUN_0000274e` | 40 | `function_0000274E_FUN_0000274e.c` |
| 0x2776 | `FUN_00002776` | 2 | `function_00002776_FUN_00002776.c` |
| 0x2778 | `FUN_00002778` | 30 | `function_00002778_FUN_00002778.c` |
| 0x36B0 | `FUN_000036b0` | 8 | `function_000036B0_FUN_000036b0.c` |
| 0x36D8 | `FUN_000036d8` | 13 | `function_000036D8_FUN_000036d8.c` |
| 0x36E6 | `FUN_000036e6` | 28 | `function_000036E6_FUN_000036e6.c` |
| 0x3702 | `thunk_FUN_0000383c` | 4 | `function_00003702_thunk_FUN_0000383c.c` |
| 0x3718 | `thunk_FUN_0000383c` | 4 | `function_00003718_thunk_FUN_0000383c.c` |
| 0x3720 | `caseD_0` | 10 | `function_00003720_caseD_0.c` |
| 0x3726 | `thunk_FUN_0000383c` | 4 | `function_00003726_thunk_FUN_0000383c.c` |
| 0x3808 | `FUN_00003808` | 4 | `function_00003808_FUN_00003808.c` |
| 0x3814 | `FUN_00003814` | 10 | `function_00003814_FUN_00003814.c` |
| 0x3830 | `caseD_20` | 2 | `function_00003830_caseD_20.c` |
| 0x383C | `FUN_0000383c` | 32 | `function_0000383C_FUN_0000383c.c` |
| 0x3C88 | `thunk_FUN_0000383c` | 4 | `function_00003C88_thunk_FUN_0000383c.c` |
| 0x3CCA | `FUN_00003cca` | 38 | `function_00003CCA_FUN_00003cca.c` |
| 0x4064 | `thunk_FUN_00004080` | 2 | `function_00004064_thunk_FUN_00004080.c` |
| 0x4080 | `FUN_00004080` | 12 | `function_00004080_FUN_00004080.c` |
| 0x408C | `FUN_0000408c` | 50 | `function_0000408C_FUN_0000408c.c` |
| 0x40BA | `FUN_000040ba` | 18 | `function_000040BA_FUN_000040ba.c` |
| 0x40CC | `FUN_000040cc` | 18 | `function_000040CC_FUN_000040cc.c` |
| 0x40E2 | `FUN_000040e2` | 259 | `function_000040E2_FUN_000040e2.c` |
| 0x4228 | `FUN_00004228` | 18 | `function_00004228_FUN_00004228.c` |
| 0x423A | `FUN_0000423a` | 260 | `function_0000423A_FUN_0000423a.c` |
| 0x437C | `FUN_0000437c` | 56 | `function_0000437C_FUN_0000437c.c` |
| 0x43B4 | `FUN_000043b4` | 20 | `function_000043B4_FUN_000043b4.c` |
| 0x43C8 | `FUN_000043c8` | 68 | `function_000043C8_FUN_000043c8.c` |
| 0x451C | `FUN_0000451c` | 16 | `function_0000451C_FUN_0000451c.c` |
| 0x4530 | `FUN_00004530` | 83 | `function_00004530_FUN_00004530.c` |
| 0x45AC | `FUN_000045ac` | 40 | `function_000045AC_FUN_000045ac.c` |
| 0x45D4 | `FUN_000045d4` | 20 | `function_000045D4_FUN_000045d4.c` |
| 0x4602 | `FUN_00004602` | 26 | `function_00004602_FUN_00004602.c` |
| 0x461C | `FUN_0000461c` | 46 | `function_0000461C_FUN_0000461c.c` |
| 0x464A | `FUN_0000464a` | 4 | `function_0000464A_FUN_0000464a.c` |
| 0x464E | `FUN_0000464e` | 40 | `function_0000464E_FUN_0000464e.c` |
| 0x4676 | `FUN_00004676` | 98 | `function_00004676_FUN_00004676.c` |
| 0x4794 | `FUN_00004794` | 24 | `function_00004794_FUN_00004794.c` |
| 0x47CC | `FUN_000047cc` | 4 | `function_000047CC_FUN_000047cc.c` |
| 0x47F4 | `FUN_000047f4` | 198 | `function_000047F4_FUN_000047f4.c` |
| 0x4980 | `FUN_00004980` | 76 | `function_00004980_FUN_00004980.c` |
| 0x49A2 | `FUN_000049a2` | 36 | `function_000049A2_FUN_000049a2.c` |
| 0x49EC | `FUN_000049ec` | 60 | `function_000049EC_FUN_000049ec.c` |
| 0x4B68 | `FUN_00004b68` | 46 | `function_00004B68_FUN_00004b68.c` |
| 0x4CE2 | `thunk_FUN_0000383c` | 4 | `function_00004CE2_thunk_FUN_0000383c.c` |
| 0x4D3E | `FUN_00004d3e` | 14 | `function_00004D3E_FUN_00004d3e.c` |
| 0x4D58 | `FUN_00004d58` | 14 | `function_00004D58_FUN_00004d58.c` |
| 0x4D66 | `FUN_00004d66` | 12 | `function_00004D66_FUN_00004d66.c` |
| 0x4FD2 | `FUN_00004fd2` | 10 | `function_00004FD2_FUN_00004fd2.c` |
| 0x50A8 | `FUN_000050a8` | 6 | `function_000050A8_FUN_000050a8.c` |
| 0x50D0 | `FUN_000050d0` | 258 | `function_000050D0_FUN_000050d0.c` |
| 0x53B6 | `FUN_000053b6` | 22 | `function_000053B6_FUN_000053b6.c` |
| 0x53CC | `FUN_000053cc` | 206 | `function_000053CC_FUN_000053cc.c` |
| 0x5416 | `FUN_00005416` | 12 | `function_00005416_FUN_00005416.c` |
| 0x5470 | `FUN_00005470` | 14 | `function_00005470_FUN_00005470.c` |
| 0x54B4 | `FUN_000054b4` | 38 | `function_000054B4_FUN_000054b4.c` |
| 0x54DA | `FUN_000054da` | 28 | `function_000054DA_FUN_000054da.c` |
| 0x54F6 | `FUN_000054f6` | 24 | `function_000054F6_FUN_000054f6.c` |
| 0x550E | `FUN_0000550e` | 56 | `function_0000550E_FUN_0000550e.c` |
| 0x551E | `FUN_0000551e` | 50 | `function_0000551E_FUN_0000551e.c` |
| 0x5550 | `FUN_00005550` | 66 | `function_00005550_FUN_00005550.c` |
| 0x55F2 | `FUN_000055f2` | 24 | `function_000055F2_FUN_000055f2.c` |
| 0x560A | `FUN_0000560a` | 36 | `function_0000560A_FUN_0000560a.c` |
| 0x562E | `FUN_0000562e` | 40 | `function_0000562E_FUN_0000562e.c` |
| 0x56E8 | `FUN_000056e8` | 14 | `function_000056E8_FUN_000056e8.c` |
| 0x56F6 | `FUN_000056f6` | 36 | `function_000056F6_FUN_000056f6.c` |
| 0x574E | `FUN_0000574e` | 6 | `function_0000574E_FUN_0000574e.c` |
| 0x5754 | `FUN_00005754` | 622 | `function_00005754_FUN_00005754.c` |
| 0x59C4 | `FUN_000059c4` | 22 | `function_000059C4_FUN_000059c4.c` |
| 0x5A58 | `FUN_00005a58` | 6 | `function_00005A58_FUN_00005a58.c` |
| 0x5A5E | `FUN_00005a5e` | 4 | `function_00005A5E_FUN_00005a5e.c` |
| 0x5A62 | `FUN_00005a62` | 4 | `function_00005A62_FUN_00005a62.c` |
| 0x5A66 | `FUN_00005a66` | 8 | `function_00005A66_FUN_00005a66.c` |
| 0x5A6E | `FUN_00005a6e` | 6 | `function_00005A6E_FUN_00005a6e.c` |
| 0x5A74 | `FUN_00005a74` | 15 | `function_00005A74_FUN_00005a74.c` |
| 0x5A94 | `FUN_00005a94` | 10 | `function_00005A94_FUN_00005a94.c` |
| 0x5AA2 | `FUN_00005aa2` | 24 | `function_00005AA2_FUN_00005aa2.c` |
| 0x5ABA | `FUN_00005aba` | 16 | `function_00005ABA_FUN_00005aba.c` |
| 0x5AEE | `FUN_00005aee` | 78 | `function_00005AEE_FUN_00005aee.c` |
| 0x5B3C | `FUN_00005b3c` | 114 | `function_00005B3C_FUN_00005b3c.c` |
| 0x5BBE | `FUN_00005bbe` | 76 | `function_00005BBE_FUN_00005bbe.c` |
| 0x5C0C | `FUN_00005c0c` | 46 | `function_00005C0C_FUN_00005c0c.c` |
| 0x5C3A | `FUN_00005c3a` | 4 | `function_00005C3A_FUN_00005c3a.c` |
| 0x5C3E | `FUN_00005c3e` | 74 | `function_00005C3E_FUN_00005c3e.c` |
| 0x6196 | `FUN_00006196` | 66 | `function_00006196_FUN_00006196.c` |
| 0x61DA | `FUN_000061da` | 166 | `function_000061DA_FUN_000061da.c` |
| 0x6280 | `FUN_00006280` | 22 | `function_00006280_FUN_00006280.c` |
| 0x6296 | `FUN_00006296` | 18 | `function_00006296_FUN_00006296.c` |
| 0x62A8 | `FUN_000062a8` | 26 | `function_000062A8_FUN_000062a8.c` |
| 0x62C2 | `FUN_000062c2` | 46 | `function_000062C2_FUN_000062c2.c` |
| 0x62F4 | `FUN_000062f4` | 70 | `function_000062F4_FUN_000062f4.c` |
| 0x633A | `FUN_0000633a` | 52 | `function_0000633A_FUN_0000633a.c` |
| 0x636E | `FUN_0000636e` | 46 | `function_0000636E_FUN_0000636e.c` |
| 0x63CA | `FUN_000063ca` | 110 | `function_000063CA_FUN_000063ca.c` |
| 0x6410 | `FUN_00006410` | 36 | `function_00006410_FUN_00006410.c` |
| 0x6436 | `FUN_00006436` | 70 | `function_00006436_FUN_00006436.c` |
| 0x64A8 | `FUN_000064a8` | 74 | `function_000064A8_FUN_000064a8.c` |
| 0x64F2 | `FUN_000064f2` | 82 | `function_000064F2_FUN_000064f2.c` |
| 0x6570 | `FUN_00006570` | 54 | `function_00006570_FUN_00006570.c` |
| 0x664E | `FUN_0000664e` | 26 | `function_0000664E_FUN_0000664e.c` |
| 0x66AE | `thunk_FUN_0000383c` | 4 | `function_000066AE_thunk_FUN_0000383c.c` |
| 0x673E | `FUN_0000673e` | 32 | `function_0000673E_FUN_0000673e.c` |
| 0x67E6 | `thunk_FUN_0000383c` | 4 | `function_000067E6_thunk_FUN_0000383c.c` |
| 0x680C | `thunk_FUN_0000383c` | 4 | `function_0000680C_thunk_FUN_0000383c.c` |
| 0x6810 | `FUN_00006810` | 44 | `function_00006810_FUN_00006810.c` |
| 0x683C | `FUN_0000683c` | 62 | `function_0000683C_FUN_0000683c.c` |
| 0x68F8 | `FUN_000068f8` | 14 | `function_000068F8_FUN_000068f8.c` |
| 0x69AA | `FUN_000069aa` | 1298 | `function_000069AA_FUN_000069aa.c` |
| 0x69B4 | `FUN_000069b4` | 4 | `function_000069B4_FUN_000069b4.c` |
| 0x69B8 | `FUN_000069b8` | 10 | `function_000069B8_FUN_000069b8.c` |
| 0x69C2 | `FUN_000069c2` | 12 | `function_000069C2_FUN_000069c2.c` |
| 0x69D0 | `FUN_000069d0` | 2 | `function_000069D0_FUN_000069d0.c` |
| 0x69D2 | `FUN_000069d2` | 10 | `function_000069D2_FUN_000069d2.c` |
| 0x69DC | `FUN_000069dc` | 4 | `function_000069DC_FUN_000069dc.c` |
| 0x69E0 | `FUN_000069e0` | 10 | `function_000069E0_FUN_000069e0.c` |
| 0x69EA | `FUN_000069ea` | 62 | `function_000069EA_FUN_000069ea.c` |
| 0x6A28 | `FUN_00006a28` | 160 | `function_00006A28_FUN_00006a28.c` |
| 0x6A50 | `FUN_00006a50` | 6 | `function_00006A50_FUN_00006a50.c` |
| 0x6A56 | `FUN_00006a56` | 12 | `function_00006A56_FUN_00006a56.c` |
| 0x6A82 | `caseD_20` | 4 | `function_00006A82_caseD_20.c` |
| 0x6A88 | `thunk_FUN_000056f6` | 4 | `function_00006A88_thunk_FUN_000056f6.c` |
| 0x6A8C | `FUN_00006a8c` | 4 | `function_00006A8C_FUN_00006a8c.c` |
| 0x6A94 | `FUN_00006a94` | 12 | `function_00006A94_FUN_00006a94.c` |
| 0x6AAC | `FUN_00006aac` | 4 | `function_00006AAC_FUN_00006aac.c` |
| 0x6AB0 | `FUN_00006ab0` | 16 | `function_00006AB0_FUN_00006ab0.c` |
| 0x6AC0 | `FUN_00006ac0` | 302 | `function_00006AC0_FUN_00006ac0.c` |
| 0x6BDE | `FUN_00006bde` | 48 | `function_00006BDE_FUN_00006bde.c` |
| 0x6C50 | `FUN_00006c50` | 36 | `function_00006C50_FUN_00006c50.c` |
| 0x6C74 | `FUN_00006c74` | 538 | `function_00006C74_FUN_00006c74.c` |
| 0x6CBC | `FUN_00006cbc` | 56 | `function_00006CBC_FUN_00006cbc.c` |
| 0x6E68 | `FUN_00006e68` | 96 | `function_00006E68_FUN_00006e68.c` |
| 0x6EC4 | `FUN_00006ec4` | 30 | `function_00006EC4_FUN_00006ec4.c` |
| 0x6EFA | `FUN_00006efa` | 20 | `function_00006EFA_FUN_00006efa.c` |
| 0x752C | `FUN_0000752c` | 102 | `function_0000752C_FUN_0000752c.c` |
| 0x79E2 | `FUN_000079e2` | 30 | `function_000079E2_FUN_000079e2.c` |
| 0x7A2A | `FUN_00007a2a` | 40 | `function_00007A2A_FUN_00007a2a.c` |
| 0x7A52 | `FUN_00007a52` | 292 | `function_00007A52_FUN_00007a52.c` |
| 0x7BFE | `FUN_00007bfe` | 64 | `function_00007BFE_FUN_00007bfe.c` |
| 0x7C3E | `FUN_00007c3e` | 38 | `function_00007C3E_FUN_00007c3e.c` |
| 0x7C64 | `FUN_00007c64` | 22 | `function_00007C64_FUN_00007c64.c` |
| 0x7CB2 | `FUN_00007cb2` | 30 | `function_00007CB2_FUN_00007cb2.c` |
| 0x7CDA | `thunk_FUN_0000383c` | 4 | `function_00007CDA_thunk_FUN_0000383c.c` |
| 0x7D58 | `FUN_00007d58` | 8 | `function_00007D58_FUN_00007d58.c` |
| 0x8052 | `FUN_00008052` | 12 | `function_00008052_FUN_00008052.c` |
| 0x805E | `FUN_0000805e` | 6 | `function_0000805E_FUN_0000805e.c` |
| 0x8064 | `FUN_00008064` | 92 | `function_00008064_FUN_00008064.c` |
| 0x80C0 | `FUN_000080c0` | 92 | `function_000080C0_FUN_000080c0.c` |
| 0x811C | `FUN_0000811c` | 122 | `function_0000811C_FUN_0000811c.c` |
| 0x824A | `FUN_0000824a` | 22 | `function_0000824A_FUN_0000824a.c` |
| 0x86DC | `FUN_000086dc` | 42 | `function_000086DC_FUN_000086dc.c` |
| 0x870A | `FUN_0000870a` | 236 | `function_0000870A_FUN_0000870a.c` |
| 0x87E2 | `FUN_000087e2` | 30 | `function_000087E2_FUN_000087e2.c` |
| 0x8812 | `FUN_00008812` | 18 | `function_00008812_FUN_00008812.c` |
| 0x883A | `FUN_0000883a` | 48 | `function_0000883A_FUN_0000883a.c` |
| 0x889C | `FUN_0000889c` | 84 | `function_0000889C_FUN_0000889c.c` |
| 0x8C0E | `FUN_00008c0e` | 24 | `function_00008C0E_FUN_00008c0e.c` |
| 0x8C46 | `FUN_00008c46` | 268 | `function_00008C46_FUN_00008c46.c` |
| 0x8D6A | `FUN_00008d6a` | 22 | `function_00008D6A_FUN_00008d6a.c` |
| 0x8FFC | `FUN_00008ffc` | 16 | `function_00008FFC_FUN_00008ffc.c` |
| 0x92C8 | `FUN_000092c8` | 8 | `function_000092C8_FUN_000092c8.c` |
| 0x92D0 | `FUN_000092d0` | 82 | `function_000092D0_FUN_000092d0.c` |
| 0x9344 | `FUN_00009344` | 32 | `function_00009344_FUN_00009344.c` |
| 0x9364 | `FUN_00009364` | 54 | `function_00009364_FUN_00009364.c` |
| 0x939A | `FUN_0000939a` | 4 | `function_0000939A_FUN_0000939a.c` |
| 0x939E | `FUN_0000939e` | 152 | `function_0000939E_FUN_0000939e.c` |
| 0x93FE | `FUN_000093fe` | 52 | `function_000093FE_FUN_000093fe.c` |
| 0x9432 | `FUN_00009432` | 50 | `function_00009432_FUN_00009432.c` |
| 0x94A6 | `FUN_000094a6` | 196 | `function_000094A6_FUN_000094a6.c` |
| 0x956A | `FUN_0000956a` | 74 | `function_0000956A_FUN_0000956a.c` |
| 0x95B4 | `FUN_000095b4` | 62 | `function_000095B4_FUN_000095b4.c` |
| 0x976E | `thunk_FUN_00000a26` | 6 | `function_0000976E_thunk_FUN_00000a26.c` |
| 0x98D8 | `FUN_000098d8` | 46 | `function_000098D8_FUN_000098d8.c` |
| 0x9906 | `FUN_00009906` | 32 | `function_00009906_FUN_00009906.c` |
| 0x99B6 | `FUN_000099b6` | 214 | `function_000099B6_FUN_000099b6.c` |
| 0x9B36 | `FUN_00009b36` | 24 | `function_00009B36_FUN_00009b36.c` |
| 0x9BF4 | `FUN_00009bf4` | 60 | `function_00009BF4_FUN_00009bf4.c` |
| 0x9C30 | `FUN_00009c30` | 40 | `function_00009C30_FUN_00009c30.c` |
| 0x9C58 | `FUN_00009c58` | 110 | `function_00009C58_FUN_00009c58.c` |
| 0x9CC6 | `FUN_00009cc6` | 20 | `function_00009CC6_FUN_00009cc6.c` |
| 0x9CEA | `FUN_00009cea` | 4 | `function_00009CEA_FUN_00009cea.c` |
| 0x9CEE | `FUN_00009cee` | 126 | `function_00009CEE_FUN_00009cee.c` |
| 0x9D6C | `FUN_00009d6c` | 40 | `function_00009D6C_FUN_00009d6c.c` |
| 0x9D94 | `FUN_00009d94` | 74 | `function_00009D94_FUN_00009d94.c` |
| 0x9DE4 | `FUN_00009de4` | 20 | `function_00009DE4_FUN_00009de4.c` |
| 0x9F28 | `FUN_00009f28` | 8 | `function_00009F28_FUN_00009f28.c` |
| 0x9F30 | `FUN_00009f30` | 142 | `function_00009F30_FUN_00009f30.c` |
| 0xA04E | `FUN_0000a04e` | 28 | `function_0000A04E_FUN_0000a04e.c` |
| 0xA0BE | `FUN_0000a0be` | 42 | `function_0000A0BE_FUN_0000a0be.c` |
| 0xA518 | `FUN_0000a518` | 82 | `function_0000A518_FUN_0000a518.c` |
| 0xA77C | `FUN_0000a77c` | 54 | `function_0000A77C_FUN_0000a77c.c` |
| 0xA7AE | `FUN_0000a7ae` | 32 | `function_0000A7AE_FUN_0000a7ae.c` |
| 0xA7CE | `FUN_0000a7ce` | 36 | `function_0000A7CE_FUN_0000a7ce.c` |
| 0xA9D0 | `FUN_0000a9d0` | 28 | `function_0000A9D0_FUN_0000a9d0.c` |
| 0xA9F8 | `FUN_0000a9f8` | 72 | `function_0000A9F8_FUN_0000a9f8.c` |
| 0xAA4A | `FUN_0000aa4a` | 6 | `function_0000AA4A_FUN_0000aa4a.c` |
| 0xAA84 | `FUN_0000aa84` | 6 | `function_0000AA84_FUN_0000aa84.c` |
| 0xAABE | `FUN_0000aabe` | 6 | `function_0000AABE_FUN_0000aabe.c` |
| 0xAAF8 | `FUN_0000aaf8` | 6 | `function_0000AAF8_FUN_0000aaf8.c` |
| 0xAAFE | `FUN_0000aafe` | 8 | `function_0000AAFE_FUN_0000aafe.c` |
| 0xABF0 | `FUN_0000abf0` | 46 | `function_0000ABF0_FUN_0000abf0.c` |
| 0xAF8A | `FUN_0000af8a` | 94 | `function_0000AF8A_FUN_0000af8a.c` |
| 0xAFE8 | `FUN_0000afe8` | 16 | `function_0000AFE8_FUN_0000afe8.c` |
| 0xAFF8 | `FUN_0000aff8` | 52 | `function_0000AFF8_FUN_0000aff8.c` |
| 0xB02C | `FUN_0000b02c` | 8 | `function_0000B02C_FUN_0000b02c.c` |
| 0xB044 | `FUN_0000b044` | 26 | `function_0000B044_FUN_0000b044.c` |
| 0xB230 | `FUN_0000b230` | 8 | `function_0000B230_FUN_0000b230.c` |
| 0xB2D2 | `FUN_0000b2d2` | 6 | `function_0000B2D2_FUN_0000b2d2.c` |
| 0xB2D8 | `FUN_0000b2d8` | 14 | `function_0000B2D8_FUN_0000b2d8.c` |
| 0xB39A | `thunk_FUN_00007a2a` | 6 | `function_0000B39A_thunk_FUN_00007a2a.c` |
| 0xB43C | `FUN_0000b43c` | 6 | `function_0000B43C_FUN_0000b43c.c` |
| 0xB934 | `FUN_0000b934` | 28 | `function_0000B934_FUN_0000b934.c` |
| 0xBA22 | `FUN_0000ba22` | 14 | `function_0000BA22_FUN_0000ba22.c` |
| 0xBA2C | `FUN_0000ba2c` | 10 | `function_0000BA2C_FUN_0000ba2c.c` |
| 0xBA4A | `FUN_0000ba4a` | 158 | `function_0000BA4A_FUN_0000ba4a.c` |
| 0xBAE8 | `FUN_0000bae8` | 10 | `function_0000BAE8_FUN_0000bae8.c` |
| 0xBAF2 | `FUN_0000baf2` | 4 | `function_0000BAF2_FUN_0000baf2.c` |
| 0xBAF6 | `FUN_0000baf6` | 36 | `function_0000BAF6_FUN_0000baf6.c` |
| 0xBB1A | `FUN_0000bb1a` | 73 | `function_0000BB1A_FUN_0000bb1a.c` |
| 0xBB64 | `FUN_0000bb64` | 32 | `function_0000BB64_FUN_0000bb64.c` |
| 0xBC4E | `FUN_0000bc4e` | 14 | `function_0000BC4E_FUN_0000bc4e.c` |
| 0xBE0C | `FUN_0000be0c` | 14 | `function_0000BE0C_FUN_0000be0c.c` |
| 0xBE1A | `FUN_0000be1a` | 22 | `function_0000BE1A_FUN_0000be1a.c` |
| 0xBE30 | `FUN_0000be30` | 54 | `function_0000BE30_FUN_0000be30.c` |
| 0xBE66 | `FUN_0000be66` | 62 | `function_0000BE66_FUN_0000be66.c` |
| 0xC5E0 | `FUN_0000c5e0` | 166 | `function_0000C5E0_FUN_0000c5e0.c` |
| 0xC764 | `FUN_0000c764` | 64 | `function_0000C764_FUN_0000c764.c` |
| 0xCA0A | `FUN_0000ca0a` | 58 | `function_0000CA0A_FUN_0000ca0a.c` |
| 0xCE02 | `FUN_0000ce02` | 178 | `function_0000CE02_FUN_0000ce02.c` |
| 0xCEB0 | `FUN_0000ceb0` | 40 | `function_0000CEB0_FUN_0000ceb0.c` |
| 0xCED8 | `FUN_0000ced8` | 4 | `function_0000CED8_FUN_0000ced8.c` |
| 0xCEDC | `FUN_0000cedc` | 32 | `function_0000CEDC_FUN_0000cedc.c` |
| 0xCF04 | `FUN_0000cf04` | 156 | `function_0000CF04_FUN_0000cf04.c` |
| 0xCFA0 | `FUN_0000cfa0` | 58 | `function_0000CFA0_FUN_0000cfa0.c` |
| 0xCFDA | `FUN_0000cfda` | 36 | `function_0000CFDA_FUN_0000cfda.c` |
| 0xD06A | `FUN_0000d06a` | 42 | `function_0000D06A_FUN_0000d06a.c` |
| 0xD0C8 | `FUN_0000d0c8` | 20 | `function_0000D0C8_FUN_0000d0c8.c` |
| 0xD154 | `FUN_0000d154` | 54 | `function_0000D154_FUN_0000d154.c` |
| 0xD18A | `FUN_0000d18a` | 32 | `function_0000D18A_FUN_0000d18a.c` |
| 0xD292 | `FUN_0000d292` | 34 | `function_0000D292_FUN_0000d292.c` |
| 0xD316 | `thunk_FUN_000122a6` | 6 | `function_0000D316_thunk_FUN_000122a6.c` |
| 0xD698 | `FUN_0000d698` | 10 | `function_0000D698_FUN_0000d698.c` |
| 0xD6A2 | `FUN_0000d6a2` | 36 | `function_0000D6A2_FUN_0000d6a2.c` |
| 0xD8F2 | `FUN_0000d8f2` | 36 | `function_0000D8F2_FUN_0000d8f2.c` |
| 0xD9CE | `FUN_0000d9ce` | 66 | `function_0000D9CE_FUN_0000d9ce.c` |
| 0xDE14 | `FUN_0000de14` | 228 | `function_0000DE14_FUN_0000de14.c` |
| 0xDF22 | `FUN_0000df22` | 36 | `function_0000DF22_FUN_0000df22.c` |
| 0xE0A2 | `FUN_0000e0a2` | 50 | `function_0000E0A2_FUN_0000e0a2.c` |
| 0xE502 | `FUN_0000e502` | 52 | `function_0000E502_FUN_0000e502.c` |
| 0xE594 | `FUN_0000e594` | 80 | `function_0000E594_FUN_0000e594.c` |
| 0xE61A | `FUN_0000e61a` | 16 | `function_0000E61A_FUN_0000e61a.c` |
| 0xE654 | `FUN_0000e654` | 10 | `function_0000E654_FUN_0000e654.c` |
| 0xE752 | `FUN_0000e752` | 32 | `function_0000E752_FUN_0000e752.c` |
| 0xE772 | `FUN_0000e772` | 4 | `function_0000E772_FUN_0000e772.c` |
| 0xE776 | `FUN_0000e776` | 32 | `function_0000E776_FUN_0000e776.c` |
| 0xE796 | `FUN_0000e796` | 136 | `function_0000E796_FUN_0000e796.c` |
| 0xE81E | `FUN_0000e81e` | 22 | `function_0000E81E_FUN_0000e81e.c` |
| 0xE834 | `FUN_0000e834` | 10 | `function_0000E834_FUN_0000e834.c` |
| 0xE86A | `FUN_0000e86a` | 22 | `function_0000E86A_FUN_0000e86a.c` |
| 0xE884 | `FUN_0000e884` | 52 | `function_0000E884_FUN_0000e884.c` |
| 0xE8B8 | `FUN_0000e8b8` | 160 | `function_0000E8B8_FUN_0000e8b8.c` |
| 0xE958 | `FUN_0000e958` | 418 | `function_0000E958_FUN_0000e958.c` |
| 0xEB2E | `FUN_0000eb2e` | 56 | `function_0000EB2E_FUN_0000eb2e.c` |
| 0xEB62 | `FUN_0000eb62` | 48 | `function_0000EB62_FUN_0000eb62.c` |
| 0xEBB2 | `FUN_0000ebb2` | 24 | `function_0000EBB2_FUN_0000ebb2.c` |
| 0xEBCA | `FUN_0000ebca` | 40 | `function_0000EBCA_FUN_0000ebca.c` |
| 0xEC78 | `FUN_0000ec78` | 96 | `function_0000EC78_FUN_0000ec78.c` |
| 0xEEDA | `FUN_0000eeda` | 90 | `function_0000EEDA_FUN_0000eeda.c` |
| 0xF01E | `FUN_0000f01e` | 72 | `function_0000F01E_FUN_0000f01e.c` |
| 0xF0A2 | `FUN_0000f0a2` | 44 | `function_0000F0A2_FUN_0000f0a2.c` |
| 0xF102 | `FUN_0000f102` | 224 | `function_0000F102_FUN_0000f102.c` |
| 0xF3CE | `FUN_0000f3ce` | 198 | `function_0000F3CE_FUN_0000f3ce.c` |
| 0xF490 | `FUN_0000f490` | 14 | `function_0000F490_FUN_0000f490.c` |
| 0xF49E | `FUN_0000f49e` | 2 | `function_0000F49E_FUN_0000f49e.c` |
| 0xF4A4 | `FUN_0000f4a4` | 26 | `function_0000F4A4_FUN_0000f4a4.c` |
| 0xF5F0 | `FUN_0000f5f0` | 1382 | `function_0000F5F0_FUN_0000f5f0.c` |
| 0xF6BA | `FUN_0000f6ba` | 24 | `function_0000F6BA_FUN_0000f6ba.c` |
| 0xF6CA | `FUN_0000f6ca` | 30 | `function_0000F6CA_FUN_0000f6ca.c` |
| 0xFD62 | `FUN_0000fd62` | 46 | `function_0000FD62_FUN_0000fd62.c` |
| 0xFD90 | `FUN_0000fd90` | 540 | `function_0000FD90_FUN_0000fd90.c` |
| 0xFE26 | `FUN_0000fe26` | 68 | `function_0000FE26_FUN_0000fe26.c` |
| 0xFE6A | `thunk_FUN_0001197e` | 6 | `function_0000FE6A_thunk_FUN_0001197e.c` |
| 0xFFEC | `FUN_0000ffec` | 82 | `function_0000FFEC_FUN_0000ffec.c` |
| 0x1003E | `FUN_0001003e` | 38 | `function_0001003E_FUN_0001003e.c` |
| 0x10064 | `FUN_00010064` | 4 | `function_00010064_FUN_00010064.c` |
| 0x10068 | `FUN_00010068` | 40 | `function_00010068_FUN_00010068.c` |
| 0x10132 | `FUN_00010132` | 180 | `function_00010132_FUN_00010132.c` |
| 0x101E6 | `FUN_000101e6` | 66 | `function_000101E6_FUN_000101e6.c` |
| 0x1026A | `FUN_0001026a` | 132 | `function_0001026A_FUN_0001026a.c` |
| 0x102F6 | `FUN_000102f6` | 4 | `function_000102F6_FUN_000102f6.c` |
| 0x102FA | `FUN_000102fa` | 236 | `function_000102FA_FUN_000102fa.c` |
| 0x103E6 | `FUN_000103e6` | 8 | `function_000103E6_FUN_000103e6.c` |
| 0x103EE | `FUN_000103ee` | 20 | `function_000103EE_FUN_000103ee.c` |
| 0x10402 | `FUN_00010402` | 180 | `function_00010402_FUN_00010402.c` |
| 0x104B6 | `FUN_000104b6` | 114 | `function_000104B6_FUN_000104b6.c` |
| 0x105D8 | `FUN_000105d8` | 6 | `function_000105D8_FUN_000105d8.c` |
| 0x10942 | `FUN_00010942` | 96 | `function_00010942_FUN_00010942.c` |
| 0x109AC | `FUN_000109ac` | 30 | `function_000109AC_FUN_000109ac.c` |
| 0x109CA | `FUN_000109ca` | 23 | `function_000109CA_FUN_000109ca.c` |
| 0x10A54 | `FUN_00010a54` | 8 | `function_00010A54_FUN_00010a54.c` |
| 0x10A5C | `FUN_00010a5c` | 4 | `function_00010A5C_FUN_00010a5c.c` |
| 0x10A60 | `FUN_00010a60` | 8 | `function_00010A60_FUN_00010a60.c` |
| 0x10A68 | `FUN_00010a68` | 2 | `function_00010A68_FUN_00010a68.c` |
| 0x10A6A | `FUN_00010a6a` | 14 | `function_00010A6A_FUN_00010a6a.c` |
| 0x10A78 | `FUN_00010a78` | 72 | `function_00010A78_FUN_00010a78.c` |
| 0x10AC0 | `FUN_00010ac0` | 28 | `function_00010AC0_FUN_00010ac0.c` |
| 0x10ADC | `FUN_00010adc` | 32 | `function_00010ADC_FUN_00010adc.c` |
| 0x10AFC | `FUN_00010afc` | 24 | `function_00010AFC_FUN_00010afc.c` |
| 0x10B14 | `FUN_00010b14` | 32 | `function_00010B14_FUN_00010b14.c` |
| 0x10B52 | `FUN_00010b52` | 54 | `function_00010B52_FUN_00010b52.c` |
| 0x10C26 | `FUN_00010c26` | 2 | `function_00010C26_FUN_00010c26.c` |
| 0x10C28 | `FUN_00010c28` | 32 | `function_00010C28_FUN_00010c28.c` |
| 0x10C48 | `FUN_00010c48` | 4 | `function_00010C48_FUN_00010c48.c` |
| 0x10C4A | `FUN_00010c4a` | 36 | `function_00010C4A_FUN_00010c4a.c` |
| 0x10C6E | `FUN_00010c6e` | 30 | `function_00010C6E_FUN_00010c6e.c` |
| 0x10C8C | `FUN_00010c8c` | 12 | `function_00010C8C_FUN_00010c8c.c` |
| 0x10C98 | `FUN_00010c98` | 174 | `function_00010C98_FUN_00010c98.c` |
| 0x10D66 | `FUN_00010d66` | 54 | `function_00010D66_FUN_00010d66.c` |
| 0x10D9C | `FUN_00010d9c` | 32 | `function_00010D9C_FUN_00010d9c.c` |
| 0x10E10 | `FUN_00010e10` | 114 | `function_00010E10_FUN_00010e10.c` |
| 0x10E82 | `FUN_00010e82` | 24 | `function_00010E82_FUN_00010e82.c` |
| 0x10E9A | `FUN_00010e9a` | 348 | `function_00010E9A_FUN_00010e9a.c` |
| 0x10EDE | `FUN_00010ede` | 8 | `function_00010EDE_FUN_00010ede.c` |
| 0x10FFE | `FUN_00010ffe` | 448 | `function_00010FFE_FUN_00010ffe.c` |
| 0x1119C | `FUN_0001119c` | 4 | `function_0001119C_FUN_0001119c.c` |
| 0x111A0 | `FUN_000111a0` | 28 | `function_000111A0_FUN_000111a0.c` |
| 0x1123A | `FUN_0001123a` | 6 | `function_0001123A_FUN_0001123a.c` |
| 0x11240 | `FUN_00011240` | 14 | `function_00011240_FUN_00011240.c` |
| 0x11428 | `FUN_00011428` | 32 | `function_00011428_FUN_00011428.c` |
| 0x114DA | `FUN_000114da` | 88 | `function_000114DA_FUN_000114da.c` |
| 0x11580 | `FUN_00011580` | 1 | `function_00011580_FUN_00011580.c` |
| 0x11588 | `FUN_00011588` | 8 | `function_00011588_FUN_00011588.c` |
| 0x11590 | `FUN_00011590` | 4 | `function_00011590_FUN_00011590.c` |
| 0x11594 | `FUN_00011594` | 10 | `function_00011594_FUN_00011594.c` |
| 0x1159E | `FUN_0001159e` | 7 | `function_0001159E_FUN_0001159e.c` |
| 0x115B0 | `FUN_000115b0` | 1 | `function_000115B0_FUN_000115b0.c` |
| 0x115D0 | `FUN_000115d0` | 1 | `function_000115D0_FUN_000115d0.c` |
| 0x115E0 | `FUN_000115e0` | 1 | `function_000115E0_FUN_000115e0.c` |
| 0x115F2 | `FUN_000115f2` | 4 | `function_000115F2_FUN_000115f2.c` |
| 0x1160E | `FUN_0001160e` | 8 | `function_0001160E_FUN_0001160e.c` |
| 0x11632 | `FUN_00011632` | 4 | `function_00011632_FUN_00011632.c` |
| 0x11644 | `FUN_00011644` | 18 | `function_00011644_FUN_00011644.c` |
| 0x11664 | `FUN_00011664` | 4 | `function_00011664_FUN_00011664.c` |
| 0x11676 | `FUN_00011676` | 4 | `function_00011676_FUN_00011676.c` |
| 0x11688 | `FUN_00011688` | 18 | `function_00011688_FUN_00011688.c` |
| 0x1169A | `FUN_0001169a` | 18 | `function_0001169A_FUN_0001169a.c` |
| 0x116BE | `FUN_000116be` | 18 | `function_000116BE_FUN_000116be.c` |
| 0x116D0 | `FUN_000116d0` | 18 | `function_000116D0_FUN_000116d0.c` |
| 0x116F4 | `FUN_000116f4` | 18 | `function_000116F4_FUN_000116f4.c` |
| 0x11732 | `FUN_00011732` | 18 | `function_00011732_FUN_00011732.c` |
| 0x11744 | `FUN_00011744` | 18 | `function_00011744_FUN_00011744.c` |
| 0x11768 | `FUN_00011768` | 18 | `function_00011768_FUN_00011768.c` |
| 0x11870 | `FUN_00011870` | 42 | `function_00011870_FUN_00011870.c` |
| 0x1189C | `FUN_0001189c` | 12 | `function_0001189C_FUN_0001189c.c` |
| 0x118AA | `FUN_000118aa` | 12 | `function_000118AA_FUN_000118aa.c` |
| 0x1195C | `FUN_0001195c` | 18 | `function_0001195C_FUN_0001195c.c` |
| 0x1197E | `FUN_0001197e` | 2 | `function_0001197E_FUN_0001197e.c` |
| 0x11984 | `FUN_00011984` | 14 | `function_00011984_FUN_00011984.c` |
| 0x119A6 | `FUN_000119a6` | 14 | `function_000119A6_FUN_000119a6.c` |
| 0x11AA8 | `FUN_00011aa8` | 14 | `function_00011AA8_FUN_00011aa8.c` |
| 0x11AC8 | `FUN_00011ac8` | 18 | `function_00011AC8_FUN_00011ac8.c` |
| 0x11B5C | `FUN_00011b5c` | 14 | `function_00011B5C_FUN_00011b5c.c` |
| 0x11B6A | `FUN_00011b6a` | 4 | `function_00011B6A_FUN_00011b6a.c` |
| 0x11B6E | `FUN_00011b6e` | 14 | `function_00011B6E_FUN_00011b6e.c` |
| 0x11B7C | `FUN_00011b7c` | 18 | `function_00011B7C_FUN_00011b7c.c` |
| 0x11B8E | `FUN_00011b8e` | 4 | `function_00011B8E_FUN_00011b8e.c` |
| 0x11B92 | `FUN_00011b92` | 14 | `function_00011B92_FUN_00011b92.c` |
| 0x11BA4 | `FUN_00011ba4` | 10 | `function_00011BA4_FUN_00011ba4.c` |
| 0x11BB6 | `FUN_00011bb6` | 10 | `function_00011BB6_FUN_00011bb6.c` |
| 0x11BF8 | `FUN_00011bf8` | 2 | `function_00011BF8_FUN_00011bf8.c` |
| 0x11C56 | `FUN_00011c56` | 16 | `function_00011C56_FUN_00011c56.c` |
| 0x11C68 | `FUN_00011c68` | 16 | `function_00011C68_FUN_00011c68.c` |
| 0x11C7A | `FUN_00011c7a` | 4 | `function_00011C7A_FUN_00011c7a.c` |
| 0x11C7E | `FUN_00011c7e` | 14 | `function_00011C7E_FUN_00011c7e.c` |
| 0x11C8C | `FUN_00011c8c` | 4 | `function_00011C8C_FUN_00011c8c.c` |
| 0x11C90 | `FUN_00011c90` | 14 | `function_00011C90_FUN_00011c90.c` |
| 0x11CA2 | `FUN_00011ca2` | 14 | `function_00011CA2_FUN_00011ca2.c` |
| 0x11E36 | `FUN_00011e36` | 36 | `function_00011E36_FUN_00011e36.c` |
| 0x11E5E | `FUN_00011e5e` | 14 | `function_00011E5E_FUN_00011e5e.c` |
| 0x11EFA | `FUN_00011efa` | 14 | `function_00011EFA_FUN_00011efa.c` |
| 0x11F30 | `FUN_00011f30` | 18 | `function_00011F30_FUN_00011f30.c` |
| 0x11F46 | `FUN_00011f46` | 14 | `function_00011F46_FUN_00011f46.c` |
| 0x11F58 | `FUN_00011f58` | 14 | `function_00011F58_FUN_00011f58.c` |
| 0x11F78 | `FUN_00011f78` | 14 | `function_00011F78_FUN_00011f78.c` |
| 0x11F8A | `FUN_00011f8a` | 18 | `function_00011F8A_FUN_00011f8a.c` |
| 0x11FD2 | `FUN_00011fd2` | 18 | `function_00011FD2_FUN_00011fd2.c` |
| 0x11FFA | `FUN_00011ffa` | 14 | `function_00011FFA_FUN_00011ffa.c` |
| 0x12034 | `FUN_00012034` | 1 | `function_00012034_FUN_00012034.c` |
| 0x12058 | `FUN_00012058` | 18 | `function_00012058_FUN_00012058.c` |
| 0x1207C | `FUN_0001207c` | 4 | `function_0001207C_FUN_0001207c.c` |
| 0x12080 | `FUN_00012080` | 14 | `function_00012080_FUN_00012080.c` |
| 0x120A0 | `FUN_000120a0` | 18 | `function_000120A0_FUN_000120a0.c` |
| 0x120B2 | `FUN_000120b2` | 18 | `function_000120B2_FUN_000120b2.c` |
| 0x120E2 | `FUN_000120e2` | 1 | `function_000120E2_FUN_000120e2.c` |
| 0x1214E | `FUN_0001214e` | 18 | `function_0001214E_FUN_0001214e.c` |
| 0x12160 | `FUN_00012160` | 18 | `function_00012160_FUN_00012160.c` |
| 0x12188 | `FUN_00012188` | 14 | `function_00012188_FUN_00012188.c` |
| 0x121A4 | `FUN_000121a4` | 18 | `function_000121A4_FUN_000121a4.c` |
| 0x121B6 | `FUN_000121b6` | 18 | `function_000121B6_FUN_000121b6.c` |
| 0x1220E | `FUN_0001220e` | 16 | `function_0001220E_FUN_0001220e.c` |
| 0x1222E | `FUN_0001222e` | 4 | `function_0001222E_FUN_0001222e.c` |
| 0x12236 | `FUN_00012236` | 10 | `function_00012236_FUN_00012236.c` |
| 0x12240 | `FUN_00012240` | 4 | `function_00012240_FUN_00012240.c` |
| 0x12268 | `FUN_00012268` | 10 | `function_00012268_FUN_00012268.c` |
| 0x122A6 | `FUN_000122a6` | 18 | `function_000122A6_FUN_000122a6.c` |
| 0x122CA | `FUN_000122ca` | 18 | `function_000122CA_FUN_000122ca.c` |
| 0x123CA | `FUN_000123ca` | 14 | `function_000123CA_FUN_000123ca.c` |
| 0x123D8 | `FUN_000123d8` | 12 | `function_000123D8_FUN_000123d8.c` |
| 0x123E4 | `FUN_000123e4` | 12 | `function_000123E4_FUN_000123e4.c` |
| 0x123F0 | `FUN_000123f0` | 12 | `function_000123F0_FUN_000123f0.c` |
| 0x123FC | `FUN_000123fc` | 12 | `function_000123FC_FUN_000123fc.c` |
| 0x12408 | `FUN_00012408` | 10 | `function_00012408_FUN_00012408.c` |
| 0x1241A | `FUN_0001241a` | 10 | `function_0001241A_FUN_0001241a.c` |
| 0x12424 | `FUN_00012424` | 10 | `function_00012424_FUN_00012424.c` |
| 0x1242E | `FUN_0001242e` | 8 | `function_0001242E_FUN_0001242e.c` |
| 0x12436 | `FUN_00012436` | 8 | `function_00012436_FUN_00012436.c` |
| 0x12452 | `FUN_00012452` | 14 | `function_00012452_FUN_00012452.c` |
| 0x12474 | `FUN_00012474` | 4 | `function_00012474_FUN_00012474.c` |
| 0x12486 | `FUN_00012486` | 2 | `function_00012486_FUN_00012486.c` |
| 0x1248C | `FUN_0001248c` | 14 | `function_0001248C_FUN_0001248c.c` |
| 0x125EC | `FUN_000125ec` | 4 | `function_000125EC_FUN_000125ec.c` |
| 0x12606 | `FUN_00012606` | 10 | `function_00012606_FUN_00012606.c` |
| 0x12620 | `FUN_00012620` | 2 | `function_00012620_FUN_00012620.c` |
| 0x12668 | `FUN_00012668` | 18 | `function_00012668_FUN_00012668.c` |
| 0x126B0 | `FUN_000126b0` | 18 | `function_000126B0_FUN_000126b0.c` |
| 0x126EE | `FUN_000126ee` | 18 | `function_000126EE_FUN_000126ee.c` |
| 0x1282E | `FUN_0001282e` | 18 | `function_0001282E_FUN_0001282e.c` |
| 0x12848 | `FUN_00012848` | 10 | `function_00012848_FUN_00012848.c` |
| 0x1287A | `FUN_0001287a` | 4 | `function_0001287A_FUN_0001287a.c` |
| 0x1289E | `FUN_0001289e` | 18 | `function_0001289E_FUN_0001289e.c` |
| 0x12946 | `FUN_00012946` | 10 | `function_00012946_FUN_00012946.c` |
| 0x12966 | `FUN_00012966` | 8 | `function_00012966_FUN_00012966.c` |
| 0x12A70 | `FUN_00012a70` | 14 | `function_00012A70_FUN_00012a70.c` |
| 0x12ABA | `FUN_00012aba` | 10 | `function_00012ABA_FUN_00012aba.c` |
| 0x12AF0 | `FUN_00012af0` | 18 | `function_00012AF0_FUN_00012af0.c` |
| 0x12B14 | `FUN_00012b14` | 18 | `function_00012B14_FUN_00012b14.c` |
| 0x12B28 | `FUN_00012b28` | 1 | `function_00012B28_FUN_00012b28.c` |
| 0x12B3A | `FUN_00012b3a` | 1 | `function_00012B3A_FUN_00012b3a.c` |
| 0x12B4C | `FUN_00012b4c` | 1 | `function_00012B4C_FUN_00012b4c.c` |
| 0x12B5E | `FUN_00012b5e` | 18 | `function_00012B5E_FUN_00012b5e.c` |
| 0x12B70 | `FUN_00012b70` | 18 | `function_00012B70_FUN_00012b70.c` |
| 0x12B82 | `FUN_00012b82` | 18 | `function_00012B82_FUN_00012b82.c` |
| 0x12B94 | `FUN_00012b94` | 18 | `function_00012B94_FUN_00012b94.c` |
| 0x12C04 | `FUN_00012c04` | 14 | `function_00012C04_FUN_00012c04.c` |
| 0x12C60 | `FUN_00012c60` | 18 | `function_00012C60_FUN_00012c60.c` |
| 0x12CD0 | `FUN_00012cd0` | 16 | `function_00012CD0_FUN_00012cd0.c` |
| 0x12DEE | `FUN_00012dee` | 18 | `function_00012DEE_FUN_00012dee.c` |
| 0x12EA2 | `FUN_00012ea2` | 18 | `function_00012EA2_FUN_00012ea2.c` |
| 0x13196 | `FUN_00013196` | 28 | `function_00013196_FUN_00013196.c` |
| 0x13AF4 | `FUN_00013af4` | 174 | `function_00013AF4_FUN_00013af4.c` |
| 0x13BA2 | `FUN_00013ba2` | 120 | `function_00013BA2_FUN_00013ba2.c` |
| 0x14056 | `FUN_00014056` | 18 | `function_00014056_FUN_00014056.c` |
| 0x141D2 | `FUN_000141d2` | 10 | `function_000141D2_FUN_000141d2.c` |
| 0x141DC | `FUN_000141dc` | 60 | `function_000141DC_FUN_000141dc.c` |
| 0x14218 | `FUN_00014218` | 334 | `function_00014218_FUN_00014218.c` |
| 0x14418 | `FUN_00014418` | 190 | `function_00014418_FUN_00014418.c` |
| 0x144D6 | `FUN_000144d6` | 120 | `function_000144D6_FUN_000144d6.c` |
| 0x1454E | `FUN_0001454e` | 18 | `function_0001454E_FUN_0001454e.c` |
| 0x14560 | `FUN_00014560` | 20 | `function_00014560_FUN_00014560.c` |
| 0x14574 | `FUN_00014574` | 2 | `function_00014574_FUN_00014574.c` |
| 0x14576 | `FUN_00014576` | 2 | `function_00014576_FUN_00014576.c` |
| 0x14578 | `FUN_00014578` | 14 | `function_00014578_FUN_00014578.c` |
| 0x14586 | `FUN_00014586` | 32 | `function_00014586_FUN_00014586.c` |
| 0x145A6 | `FUN_000145a6` | 22 | `function_000145A6_FUN_000145a6.c` |
| 0x145BC | `FUN_000145bc` | 46 | `function_000145BC_FUN_000145bc.c` |
| 0x145FA | `FUN_000145fa` | 16 | `function_000145FA_FUN_000145fa.c` |
| 0x147CC | `FUN_000147cc` | 20 | `function_000147CC_FUN_000147cc.c` |
| 0x147E0 | `FUN_000147e0` | 14 | `function_000147E0_FUN_000147e0.c` |
| 0x147EE | `FUN_000147ee` | 106 | `function_000147EE_FUN_000147ee.c` |
| 0x14858 | `FUN_00014858` | 46 | `function_00014858_FUN_00014858.c` |
| 0x14886 | `FUN_00014886` | 86 | `function_00014886_FUN_00014886.c` |
| 0x148DE | `FUN_000148de` | 22 | `function_000148DE_FUN_000148de.c` |
| 0x148F4 | `FUN_000148f4` | 18 | `function_000148F4_FUN_000148f4.c` |
| 0x14942 | `FUN_00014942` | 1 | `function_00014942_FUN_00014942.c` |
| 0x14998 | `FUN_00014998` | 68 | `function_00014998_FUN_00014998.c` |
| 0x1532C | `FUN_0001532c` | 40 | `function_0001532C_FUN_0001532c.c` |
| 0x153F2 | `FUN_000153f2` | 32 | `function_000153F2_FUN_000153f2.c` |
| 0x15518 | `FUN_00015518` | 8 | `function_00015518_FUN_00015518.c` |
| 0x15522 | `FUN_00015522` | 4 | `function_00015522_FUN_00015522.c` |
| 0x1554A | `FUN_0001554a` | 6 | `function_0001554A_FUN_0001554a.c` |
| 0x15550 | `FUN_00015550` | 28 | `function_00015550_FUN_00015550.c` |
| 0x1556C | `FUN_0001556c` | 22 | `function_0001556C_FUN_0001556c.c` |
| 0x15582 | `FUN_00015582` | 40 | `function_00015582_FUN_00015582.c` |
| 0x155AA | `FUN_000155aa` | 14 | `function_000155AA_FUN_000155aa.c` |
| 0x155B8 | `FUN_000155b8` | 16 | `function_000155B8_FUN_000155b8.c` |
| 0x1566E | `FUN_0001566e` | 272 | `function_0001566E_FUN_0001566e.c` |
| 0x1577E | `FUN_0001577e` | 4 | `function_0001577E_FUN_0001577e.c` |
| 0x15782 | `FUN_00015782` | 16 | `function_00015782_FUN_00015782.c` |
| 0x15792 | `FUN_00015792` | 36 | `function_00015792_FUN_00015792.c` |
| 0x157B6 | `FUN_000157b6` | 92 | `function_000157B6_FUN_000157b6.c` |
| 0x15828 | `FUN_00015828` | 68 | `function_00015828_FUN_00015828.c` |
| 0x15870 | `FUN_00015870` | 40 | `function_00015870_FUN_00015870.c` |
| 0x15898 | `FUN_00015898` | 18 | `function_00015898_FUN_00015898.c` |
| 0x158B6 | `FUN_000158b6` | 24 | `function_000158B6_FUN_000158b6.c` |
| 0x15C80 | `FUN_00015c80` | 66 | `function_00015C80_FUN_00015c80.c` |
| 0x15D20 | `FUN_00015d20` | 48 | `function_00015D20_FUN_00015d20.c` |
| 0x15D50 | `FUN_00015d50` | 16 | `function_00015D50_FUN_00015d50.c` |
| 0x15D60 | `FUN_00015d60` | 10 | `function_00015D60_FUN_00015d60.c` |
| 0x15D78 | `FUN_00015d78` | 41 | `function_00015D78_FUN_00015d78.c` |
| 0x15DA2 | `FUN_00015da2` | 30 | `function_00015DA2_FUN_00015da2.c` |
| 0x15E68 | `FUN_00015e68` | 1 | `function_00015E68_FUN_00015e68.c` |
| 0x15E76 | `FUN_00015e76` | 16 | `function_00015E76_FUN_00015e76.c` |
| 0x15E9E | `FUN_00015e9e` | 92 | `function_00015E9E_FUN_00015e9e.c` |
| 0x18A7E | `FUN_00018a7e` | 48 | `function_00018A7E_FUN_00018a7e.c` |
| 0x18A8C | `FUN_00018a8c` | 10 | `function_00018A8C_FUN_00018a8c.c` |
| 0x18CE6 | `FUN_00018ce6` | 62 | `function_00018CE6_FUN_00018ce6.c` |
| 0x18D34 | `FUN_00018d34` | 224 | `function_00018D34_FUN_00018d34.c` |
| 0x1B01C | `FUN_0001b01c` | 16 | `function_0001B01C_FUN_0001b01c.c` |
| 0x1B094 | `FUN_0001b094` | 92 | `function_0001B094_FUN_0001b094.c` |
| 0x1BD38 | `FUN_0001bd38` | 16 | `function_0001BD38_FUN_0001bd38.c` |
| 0x1BD5A | `FUN_0001bd5a` | 412 | `function_0001BD5A_FUN_0001bd5a.c` |
| 0x1BF30 | `FUN_0001bf30` | 46 | `function_0001BF30_FUN_0001bf30.c` |
| 0x1C11A | `FUN_0001c11a` | 104 | `function_0001C11A_FUN_0001c11a.c` |
| 0x1C270 | `FUN_0001c270` | 100 | `function_0001C270_FUN_0001c270.c` |
| 0x1C2D4 | `FUN_0001c2d4` | 56 | `function_0001C2D4_FUN_0001c2d4.c` |
| 0x1C386 | `FUN_0001c386` | 80 | `function_0001C386_FUN_0001c386.c` |
| 0x1C492 | `FUN_0001c492` | 66 | `function_0001C492_FUN_0001c492.c` |
| 0x1C4D4 | `FUN_0001c4d4` | 12 | `function_0001C4D4_FUN_0001c4d4.c` |
| 0x1C4E2 | `FUN_0001c4e2` | 12 | `function_0001C4E2_FUN_0001c4e2.c` |
| 0x1C552 | `FUN_0001c552` | 86 | `function_0001C552_FUN_0001c552.c` |
| 0x1C740 | `FUN_0001c740` | 12 | `function_0001C740_FUN_0001c740.c` |
| 0x1C7B6 | `FUN_0001c7b6` | 368 | `function_0001C7B6_FUN_0001c7b6.c` |
| 0x1CB16 | `FUN_0001cb16` | 486 | `function_0001CB16_FUN_0001cb16.c` |
| 0x1D5C6 | `FUN_0001d5c6` | 36 | `function_0001D5C6_FUN_0001d5c6.c` |
| 0x1D5EA | `FUN_0001d5ea` | 4 | `function_0001D5EA_FUN_0001d5ea.c` |
| 0x1D5EE | `FUN_0001d5ee` | 10 | `function_0001D5EE_FUN_0001d5ee.c` |
| 0x1D5FC | `FUN_0001d5fc` | 196 | `function_0001D5FC_FUN_0001d5fc.c` |
| 0x1D6C0 | `FUN_0001d6c0` | 4 | `function_0001D6C0_FUN_0001d6c0.c` |
| 0x1D6C4 | `FUN_0001d6c4` | 26 | `function_0001D6C4_FUN_0001d6c4.c` |
| 0x1D6DE | `FUN_0001d6de` | 6 | `function_0001D6DE_FUN_0001d6de.c` |
| 0x1D6E4 | `FUN_0001d6e4` | 18 | `function_0001D6E4_FUN_0001d6e4.c` |
| 0x1D79E | `FUN_0001d79e` | 26 | `function_0001D79E_FUN_0001d79e.c` |
| 0x1D97C | `FUN_0001d97c` | 834 | `function_0001D97C_FUN_0001d97c.c` |
