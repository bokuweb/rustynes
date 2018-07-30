	; マッパー３サンプル

	; INESヘッダー
	.inesprg 1 ;   - プログラム１バンク
	.ineschr 2 ;   - グラフィック２バンク
	.inesmir 1 ;   - 垂直ミラーリング
	.inesmap 3 ;   - マッパー３番(CNROM)

	; ゼロページ
ViewAdr_L = $00		; ネームテーブルアドレス(下位)
ViewAdr_H = $01		; ネームテーブルアドレス(上位)
MapAdr_L = $02		; マップアドレス(下位)
MapAdr_H = $03		; マップアドレス(下位)
MapAdrW_L = $04		; マップアドレス(下位)更新用
MapAdrW_H = $05		; マップアドレス(上位)更新用
MapAdr_ofs = $06	; マップアクセス用オフセット

View_X = $07		; 汎用
View_Y = $08		; 汎用
Work_X = $09		; 汎用
Work_Y = $0A		; 汎用

Walk_Cnt = $10
GameMode = $11		; モード(0=フィールド,1=バンク切り替え予約,2=切り替え後)

KumaAdr_L = $20		; クマ下位アドレス
KumaAdr_H = $21		; クマ上位アドレス

	.bank 1      ; バンク１
	.org $FFFA   ; $FFFAから開始

	.dw mainLoop ; VBlank割り込みハンドラ(1/60秒毎にmainLoopがコールされる)
	.dw Start    ; リセット割り込み。起動時とリセットでStartに飛ぶ
	.dw IRQ      ; ハードウェア割り込みとソフトウェア割り込みによって発生

	.bank 0		; バンク０

	.org $0300	 ; $0300から開始、スプライトDMAデータ配置
Sprite1_Y:     .db  0   ; スプライト#1 Y座標
Sprite1_T:     .db  0   ; スプライト#1 ナンバー
Sprite1_S:     .db  0   ; スプライト#1 属性
Sprite1_X:     .db  0   ; スプライト#1 X座標
Sprite2_Y:     .db  0   ; スプライト#1 Y座標
Sprite2_T:     .db  0   ; スプライト#1 ナンバー
Sprite2_S:     .db  0   ; スプライト#1 属性
Sprite2_X:     .db  0   ; スプライト#1 X座標

	.org $8000	; $8000から開始

Start:
	sei			; 割り込み不許可
	cld			; デシマルモードフラグクリア
	ldx #$ff
	txs			; スタックポインタ初期化 

	; PPUコントロールレジスタ1初期化
	lda #%00110000	; ここではVBlank割り込み禁止
	sta $2000

	; VROMバンク切り替え
	lda #2			; バンク2
	sta $8000

waitVSync:
	lda $2002		; VBlankが発生すると、$2002の7ビット目が1になる
	bpl waitVSync  	; bit7が0の間は、waitVSyncラベルの位置に飛んでループして待ち続ける

	; PPUコントロールレジスタ2初期化
	lda #%00000110	; 初期化中はスプライトとBGを表示OFFにする
	sta $2001

	; パレットをロード
	ldx #$00    	; Xレジスタクリア

	; VRAMアドレスレジスタの$2006に、パレットのロード先のアドレス$3F00を指定する。
	lda #$3F
	sta $2006
	lda #$00
	sta $2006

loadPal:			; ラベルは、「ラベル名＋:」の形式で記述
	lda tilepal, x	; Aに(ourpal + x)番地のパレットをロードする
	sta $2007		; $2007にパレットの値を読み込む
	inx				; Xレジスタに値を1加算している
	cpx #32 		; Xを32(10進数。BGとスプライトのパレットの総数)と比較して同じかどうか比較している	
	bne loadPal		;	上が等しくない場合は、loadpalラベルの位置にジャンプする
	; Xが32ならパレットロード終了

	; スプライトDMA領域初期化(すべて0にする)
	lda #0
	ldx #$00
initSpriteDMA:
	sta $0300, x
	inx
	bne initSpriteDMA

	; ゼロページ初期化
	lda #$00
	ldx #$00
initZeroPage:
	sta <$00, x
	inx
	bne initZeroPage

	; 初期地形描画
	; ViewAdr初期化($2000)
	lda #$20
	sta <ViewAdr_H
	sta $2006
	lda #$00
	sta <ViewAdr_L
	sta $2006
	; マップの先頭アドレス設定
	lda #high(Map_Tbl_Init)
	sta <MapAdr_H
	lda #low(Map_Tbl_Init)
	sta <MapAdr_L
	lda #32				; ネームテーブル1ライン16*2セット
	sta <View_Y			; とりあえずView_Yを使う
initField:
	ldy <MapAdr_ofs		; オフセット
	lda [MapAdr_L],y	; マップロード
	pha					; Aを保存
	lda <View_Y
	and #1
	bne initFieldSub	; View_Yが偶数ならネームテーブル上位、奇数なら下位

	pla					; Aに復帰
	; 上位を取得するので4回右シフトにする
	lsr a
	lsr a
	lsr a
	lsr a
	jmp initFieldSub2
initFieldSub
	; 下位取得
	pla					; Aに復帰
	and #$F	
	; 下位取得後にMapAdrオフセット加算
	inc <MapAdr_ofs
initFieldSub2
	; キャラ出力
	; 2x2キャラなので4倍にする
	asl a
	asl a
	clc
	ldy <View_Y
	cpy #17
	bcs initFieldSub3
	clc
	adc #2				; 2x2のうちの下の部分のキャラなので2加算する
initFieldSub3
	; 2キャラ出力する
	sta $2007
	clc
	adc #1				; 1加算
	sta $2007

	dec <View_Y
	lda <View_Y
	beq initFieldEnter2	; 2行描き終えた

	cmp #16
	bne initField

	; ネームテーブル改行処理(1行描き終えた)
	; MapAdrオフセットクリア
	lda #0
	sta <MapAdr_ofs
	jmp initField

initFieldEnter2
	; ネームテーブル改行処理(2行描き終えた)
	inc <View_X
	lda <View_X
	cmp #15
	beq initFieldEnd	; 15行出力したら終了

	lda #32				; ネームテーブル1ライン16*2セット
	sta <View_Y
	; MapAdr加算
	lda <MapAdr_L
	clc
	adc <MapAdr_ofs		; オフセットを加算
	adc #8				; さらに画面外の分の8をスキップ
	sta <MapAdr_L
	bcc initFieldSub4

	inc <MapAdr_H		; 桁上がり
initFieldSub4
	lda #0
	sta <MapAdr_ofs

	jmp initField

initFieldEnd
	; マップの先頭アドレスを再度設定して初期化
	lda #high(Map_Tbl_Init)
	sta <MapAdr_H
	sta <MapAdrW_H
	lda #low(Map_Tbl_Init)
	sta <MapAdr_L
	sta <MapAdrW_L

	; 属性を初期化する
	; $23C0から
	lda #$23
	sta $2006
	lda #$C0
	sta $2006
	sta <Work_X
	lda #8			; 8回毎に改行
	sta <Work_Y
initAttr
	jsr setAttrib
	sta $2007
	inc <MapAdrW_L
	lda <MapAdrW_L
	bne initAttrSub
	inc <MapAdrW_H	; 桁上がり
initAttrSub
	dec <Work_Y
	lda <Work_Y
	bne initAttrSub2
	lda #8			; 8回毎に改行
	sta <Work_Y
	lda <MapAdrW_L
	clc
	adc #24			; 8+16
	sta <MapAdrW_L
	bcc initAttrSub2
	inc <MapAdrW_H	; 桁上がり
initAttrSub2
	inc <Work_X
	lda <Work_X
	bne initAttr	; Xが$00になるまでループ

	; スクロールクリア
	lda $2002
	lda #$00
	sta $2005
	sta $2005

	; VBlank待ち
waitVSync2:
	lda $2002
	bpl waitVSync2

	; PPUコントロールレジスタ2初期化
	lda #%00011110	; BGの表示をONにする
	sta $2001

	; PPUコントロールレジスタ1の割り込み許可フラグを立てる
	lda #%10110000
	sta $2000

infinityLoop:		; VBlank割り込み発生を待つだけの無限ループ
	lda <GameMode
	cmp #1
	bne infinityLoop

	lda #%00110000	; VBlank割り込み禁止

	; VROMバンク切り替え
	lda #3			; バンク3
	sta $8000

	; ネームテーブルクリア
	jsr clearNameTbl

	; クマ表示
	jsr putKuma

waitVSync3:
	lda $2002		; VBlank待ち
	bpl waitVSync3

	lda #%00011110	; BGの表示をONにする
	sta $2001

	; スクロールクリア
	lda $2002
	lda #$00
	sta $2005
	sta $2005

	; ゲームモードを2に
	inc <GameMode

	lda #%10110000	; VBlank割込み禁止解除

	jmp infinityLoop

mainLoop:			; メインループ
	pha				; Aレジスタをスタックに保存

	; ゲームモード0のとき以外は何もしない
	lda <GameMode
	beq mainLoopSub
	pla				; 割り込み前の内容をAレジスタに復帰
	rti

mainLoopSub

	jsr putSprite

	inc <Walk_Cnt	; 歩きカウンター加算

	; パッドI/Oレジスタの準備
	lda #$01
	sta $4016
	lda #$00
	sta $4016

	; パッド入力チェック
	lda $4016  ; Aボタン
	and #1     ; AND #1
	beq NOTHINGdown
	; バンク切り替え予約
	inc <GameMode

	lda #%00000110	; スプライトとBGを表示OFFにする
	sta $2001

	jmp NOTHINGdown
NOTHINGdown:
	pla				; 割り込み前の内容をAレジスタに復帰
	rti				; 割り込みから復帰

putSprite:
	lda #$3  ; スプライトデータは$0300番地からなので、3をロードする。
	sta $4014 ; スプライトDMAレジスタにAをストアして、スプライトデータをDMA転送する

	; プレイヤーキャラスプライト描画(座標固定)

	; 歩きアニメパターン取得
	lda <Walk_Cnt
	and #$20
	asl a
	tax

	; 左側
	lda #112    ; Y座標
	sta Sprite1_Y
	cpx #$40
	beq spritePut
	lda #02     ; 2番
	jmp spritePut2
spritePut
	lda #04		; 4番
spritePut2
	sta Sprite1_T
	stx Sprite1_S
	lda #112	; X座標
	sta Sprite1_X
	; 右側
	lda #112    ; Y座標
	sta Sprite2_Y
	cpx #$40
	beq spritePut3
	lda #04     ; 4番
	jmp spritePut4
spritePut3
	lda #02		; 2番
spritePut4
	sta Sprite2_T
	stx Sprite2_S
	lda #120	; X座標
	sta Sprite2_X
	rts

setAttrib:
	; MapAdrW_H,Lを左上として、属性1マス分をAレジスタに設定する
	; (Mapのキャラ番号とパレット番号は同一というシンプルな前提)
	ldy #0		; オフセット
	; 左上(000000xx)
	lda [MapAdrW_L],y	; マップロード
	; 上位を取得するので4回右シフトにする
	ldx #4
	jsr shiftR
	sta <View_X			; View_Xに保存
	; 右上(0000xx00)
	lda [MapAdrW_L],y	; マップロード
	; 下位を取得するので$FとANDする
	and #$F
	; 左シフト2回してView_XにORする
	ldx #2
	jsr shiftL
	ora <View_X
	sta <View_X
	; 左下(00xx0000)
	ldy #16				; マップは横16バイトなので16加算
	lda [MapAdrW_L],y	; マップロード
	; 上位を取得するので4回右シフトにする
	ldx #4
	jsr shiftR
	; 左シフト4回してView_XにORする
	ldx #4
	jsr shiftL
	ora <View_X
	sta <View_X
	; 右下(xx000000)
	lda [MapAdrW_L],y	; マップロード
	; 下位を取得するので$FとANDする
	and #$F
	; 左シフト6回してView_XにORする
	ldx #6
	jsr shiftL
	ora <View_X
	rts

shiftL:
	; Xレジスタの回数だけAを左シフトする
	asl a
	dex
	bne shiftL
	rts

shiftR:
	; Xレジスタの回数だけAを右シフトする
	lsr a
	dex
	bne shiftR
	rts

putKuma:
	; プレイヤーキャラスプライトクリア
	lda #$00	; アドレス0
	sta $2003
	sta $2004	; マッパー３サンプル

	; INESヘッダー
	.inesprg 1 ;   - プログラム１バンク
	.ineschr 2 ;   - グラフィック２バンク
	.inesmir 1 ;   - 垂直ミラーリング
	.inesmap 3 ;   - マッパー３番(CNROM)

	; ゼロページ
ViewAdr_L = $00		; ネームテーブルアドレス(下位)
ViewAdr_H = $01		; ネームテーブルアドレス(上位)
MapAdr_L = $02		; マップアドレス(下位)
MapAdr_H = $03		; マップアドレス(下位)
MapAdrW_L = $04		; マップアドレス(下位)更新用
MapAdrW_H = $05		; マップアドレス(上位)更新用
MapAdr_ofs = $06	; マップアクセス用オフセット

View_X = $07		; 汎用
View_Y = $08		; 汎用
Work_X = $09		; 汎用
Work_Y = $0A		; 汎用

Walk_Cnt = $10
GameMode = $11		; モード(0=フィールド,1=バンク切り替え予約,2=切り替え後)

KumaAdr_L = $20		; クマ下位アドレス
KumaAdr_H = $21		; クマ上位アドレス

	.bank 1      ; バンク１
	.org $FFFA   ; $FFFAから開始

	.dw mainLoop ; VBlank割り込みハンドラ(1/60秒毎にmainLoopがコールされる)
	.dw Start    ; リセット割り込み。起動時とリセットでStartに飛ぶ
	.dw IRQ      ; ハードウェア割り込みとソフトウェア割り込みによって発生

	.bank 0		; バンク０

	.org $0300	 ; $0300から開始、スプライトDMAデータ配置
Sprite1_Y:     .db  0   ; スプライト#1 Y座標
Sprite1_T:     .db  0   ; スプライト#1 ナンバー
Sprite1_S:     .db  0   ; スプライト#1 属性
Sprite1_X:     .db  0   ; スプライト#1 X座標
Sprite2_Y:     .db  0   ; スプライト#1 Y座標
Sprite2_T:     .db  0   ; スプライト#1 ナンバー
Sprite2_S:     .db  0   ; スプライト#1 属性
Sprite2_X:     .db  0   ; スプライト#1 X座標

	.org $8000	; $8000から開始

Start:
	sei			; 割り込み不許可
	cld			; デシマルモードフラグクリア
	ldx #$ff
	txs			; スタックポインタ初期化 

	; PPUコントロールレジスタ1初期化
	lda #%00110000	; ここではVBlank割り込み禁止
	sta $2000

	; VROMバンク切り替え
	lda #2			; バンク2
	sta $8000

waitVSync:
	lda $2002		; VBlankが発生すると、$2002の7ビット目が1になる
	bpl waitVSync  	; bit7が0の間は、waitVSyncラベルの位置に飛んでループして待ち続ける

	; PPUコントロールレジスタ2初期化
	lda #%00000110	; 初期化中はスプライトとBGを表示OFFにする
	sta $2001

	; パレットをロード
	ldx #$00    	; Xレジスタクリア

	; VRAMアドレスレジスタの$2006に、パレットのロード先のアドレス$3F00を指定する。
	lda #$3F
	sta $2006
	lda #$00
	sta $2006

loadPal:			; ラベルは、「ラベル名＋:」の形式で記述
	lda tilepal, x	; Aに(ourpal + x)番地のパレットをロードする
	sta $2007		; $2007にパレットの値を読み込む
	inx				; Xレジスタに値を1加算している
	cpx #32 		; Xを32(10進数。BGとスプライトのパレットの総数)と比較して同じかどうか比較している	
	bne loadPal		;	上が等しくない場合は、loadpalラベルの位置にジャンプする
	; Xが32ならパレットロード終了

	; スプライトDMA領域初期化(すべて0にする)
	lda #0
	ldx #$00
initSpriteDMA:
	sta $0300, x
	inx
	bne initSpriteDMA

	; ゼロページ初期化
	lda #$00
	ldx #$00
initZeroPage:
	sta <$00, x
	inx
	bne initZeroPage

	; 初期地形描画
	; ViewAdr初期化($2000)
	lda #$20
	sta <ViewAdr_H
	sta $2006
	lda #$00
	sta <ViewAdr_L
	sta $2006
	; マップの先頭アドレス設定
	lda #high(Map_Tbl_Init)
	sta <MapAdr_H
	lda #low(Map_Tbl_Init)
	sta <MapAdr_L
	lda #32				; ネームテーブル1ライン16*2セット
	sta <View_Y			; とりあえずView_Yを使う
initField:
	ldy <MapAdr_ofs		; オフセット
	lda [MapAdr_L],y	; マップロード
	pha					; Aを保存
	lda <View_Y
	and #1
	bne initFieldSub	; View_Yが偶数ならネームテーブル上位、奇数なら下位

	pla					; Aに復帰
	; 上位を取得するので4回右シフトにする
	lsr a
	lsr a
	lsr a
	lsr a
	jmp initFieldSub2
initFieldSub
	; 下位取得
	pla					; Aに復帰
	and #$F	
	; 下位取得後にMapAdrオフセット加算
	inc <MapAdr_ofs
initFieldSub2
	; キャラ出力
	; 2x2キャラなので4倍にする
	asl a
	asl a
	clc
	ldy <View_Y
	cpy #17
	bcs initFieldSub3
	clc
	adc #2				; 2x2のうちの下の部分のキャラなので2加算する
initFieldSub3
	; 2キャラ出力する
	sta $2007
	clc
	adc #1				; 1加算
	sta $2007

	dec <View_Y
	lda <View_Y
	beq initFieldEnter2	; 2行描き終えた

	cmp #16
	bne initField

	; ネームテーブル改行処理(1行描き終えた)
	; MapAdrオフセットクリア
	lda #0
	sta <MapAdr_ofs
	jmp initField

initFieldEnter2
	; ネームテーブル改行処理(2行描き終えた)
	inc <View_X
	lda <View_X
	cmp #15
	beq initFieldEnd	; 15行出力したら終了

	lda #32				; ネームテーブル1ライン16*2セット
	sta <View_Y
	; MapAdr加算
	lda <MapAdr_L
	clc
	adc <MapAdr_ofs		; オフセットを加算
	adc #8				; さらに画面外の分の8をスキップ
	sta <MapAdr_L
	bcc initFieldSub4

	inc <MapAdr_H		; 桁上がり
initFieldSub4
	lda #0
	sta <MapAdr_ofs

	jmp initField

initFieldEnd
	; マップの先頭アドレスを再度設定して初期化
	lda #high(Map_Tbl_Init)
	sta <MapAdr_H
	sta <MapAdrW_H
	lda #low(Map_Tbl_Init)
	sta <MapAdr_L
	sta <MapAdrW_L

	; 属性を初期化する
	; $23C0から
	lda #$23
	sta $2006
	lda #$C0
	sta $2006
	sta <Work_X
	lda #8			; 8回毎に改行
	sta <Work_Y
initAttr
	jsr setAttrib
	sta $2007
	inc <MapAdrW_L
	lda <MapAdrW_L
	bne initAttrSub
	inc <MapAdrW_H	; 桁上がり
initAttrSub
	dec <Work_Y
	lda <Work_Y
	sta $2004
	sta $2004
	sta $2004
	lda #$04	; アドレス4
	sta $2003
	lda #$00
	sta $2004
	sta $2004
	sta $2004
	sta $2004

	; BGパレットとスプライトパレット書き換え
	lda #$3F
	sta $2006
	lda #$01
	sta $2006
	lda #$30	; 白
	sta $2007
	sta $2007
	lda #$0F	; 黒
	sta $2007

	lda #$3F
	sta $2006
	lda #$10
	sta $2006
	lda #$0F	; 黒
	sta $2007

	; 属性初期化
	jsr clearAttrib

	; クマ表示
	lda #$20
	sta <KumaAdr_H
	lda #$6B
	sta <KumaAdr_L
	ldx #0
putKumaSub:
	lda <KumaAdr_H
	sta $2006
	lda <KumaAdr_L
	sta $2006
putKumaSub2
	stx $2007
	inx
	txa
	and #$F
	cmp #$9
	bne putKumaSub2
	lda <KumaAdr_L
	clc
	adc #$20			; ネームテーブルを改行する
	sta <KumaAdr_L
	bcc putKumaSub3
	inc <KumaAdr_H		; 桁上がり
putKumaSub3
	txa
	clc
	adc #$7				; キャラを改行する
	tax
	cpx #$E0			; キャラを$D0まで出力する
	bne putKumaSub

	; 枠表示
	lda #$22
	sta $2006
	lda #$28
	sta $2006
	ldx #$E
	ldy #$FC			; 横線
putYokoWaku1
	sty $2007
	dex
	bne putYokoWaku1

	lda #$23
	sta $2006
	lda #$08
	sta $2006
	ldx #$E
putYokoWaku2
	sty $2007
	dex
	bne putYokoWaku2

	ldx #$47
	ldy #$FD			; 縦線
putTateWaku1
	lda #$22
	sta $2006
	stx $2006
	sty $2007
	txa
	clc
	adc #$20
	tax
	bcc putTateWaku1	; 面倒なんで桁上がりするまで

	ldx #$56
putTateWaku2
	lda #$22
	sta $2006
	stx $2006
	sty $2007
	txa
	clc
	adc #$20
	tax
	bcc putTateWaku2	; 面倒なんで桁上がりするまで

	; 左上
	lda #$22
	sta $2006
	lda #$27
	sta $2006
	lda #$EE
	sta $2007
	; 右上
	lda #$22
	sta $2006
	lda #$36
	sta $2006
	lda #$EF
	sta $2007
	; 左下
	lda #$23
	sta $2006
	lda #$07
	sta $2006
	lda #$FE
	sta $2007
	; 右下
	lda #$23
	sta $2006
	lda #$16
	sta $2006
	lda #$FF
	sta $2007

	; 「くまが　あらわれた！」
	lda #$22
	sta $2006
	lda #$48
	sta $2006
	ldx #$E0
putKumaAppeared
	stx $2007
	inx
	cpx #$E9
	bne putKumaAppeared

	rts

clearNameTbl:
	; ネームテーブル0クリア
	; ネームテーブルの$2000から
	lda #$20
	sta $2006
	lda #$00
	sta $2006
	lda #$00        ; 0番(透明)
	ldx #240		; 240回繰り返す
	ldy #4			; それを4回、計960回繰り返す
clearNameTblSub
	sta $2007
	dex
	bne clearNameTblSub
	ldx #240
	dey
	bne clearNameTblSub
	rts

clearAttrib:
	; 属性初期化
	lda #$23
	sta $2006
	lda #$C0
	sta $2006
	ldx #$00    	; Xレジスタクリア
	lda #0			; ４つともパレット0番
clearAttribSub
	sta $2007		; $2007に属性の値を読み込む
	; 64回(全キャラクター分)ループする
	inx
	cpx #64
	bne clearAttribSub
	rts

IRQ:
	rti

	; 初期データ
	.org $9000    ; $9000から開始
tilepal: .incbin "giko6.pal" ; パレットをincludeする
	; マップデータ(32x32)
Map_Tbl: .include "giko019map.txt"

	.bank 2       ; バンク２
	.org $0000    ; $0000から開始
	.incbin "giko4.spr"  ; スプライトデータ
	.incbin "giko7.bkg"  ; 地形BGデータ

	.bank 3       ; バンク３
	.org $0000    ; $0000から開始
	.incbin "giko4.spr"  ; スプライトデータ
	.incbin "giko8.bkg"  ; 敵&メッセージBGデータ