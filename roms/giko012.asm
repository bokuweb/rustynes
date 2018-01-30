	; サウンドサンプル

	; INESヘッダー
	.inesprg 1 ;   - プログラムにいくつのバンクを使うか。今は１つ。
	.ineschr 1 ;   - グラフィックデータにいくつのバンクを使うか。今は１つ。
	.inesmir 0 ;   - 水平ミラーリング
	.inesmap 0 ;   - マッパー。０番にする。

	; ゼロページ変数
Scroll_X = $00	; Xスクロール値
Sound_A = $01	; Aサウンドカウンタ
Sound_B = $02   ; Bサウンドカウンタ

	.bank 1      ; バンク１
	.org $FFFA   ; $FFFAから開始

	.dw mainLoop ; VBlank割り込みハンドラ(1/60秒毎にmainLoopがコールされる)
	.dw Start    ; リセット割り込み。起動時とリセットでStartに飛ぶ
	.dw 0        ; ハードウェア割り込みとソフトウェア割り込みによって発生

	.bank 0			 ; バンク０
	.org $0300	 ; $0300から開始、スプライトDMAデータ配置
Sprite1_Y:     .db  0   ; スプライト#1 Y座標
Sprite1_T:     .db  0   ; スプライト#1 ナンバー
Sprite1_S:     .db  0   ; スプライト#1 属性
Sprite1_X:     .db  0   ; スプライト#1 X座標
Sprite2_Y:     .db  0   ; スプライト#2 Y座標
Sprite2_T:     .db  0   ; スプライト#2 ナンバー
Sprite2_S:     .db  0   ; スプライト#2 属性
Sprite2_X:     .db  0   ; スプライト#2 X座標


	.org $8000	 ; $8000から開始
Start:
	; PPUコントロールレジスタ1初期化
	lda #%00001000	; ここではVBlank割り込み禁止
	sta $2000

waitVSync:
	lda $2002			; VBlankが発生すると、$2002の7ビット目が1になる
	bpl waitVSync  ; bit7が0の間は、waitVSyncラベルの位置に飛んでループして待ち続ける


	; PPUコントロールレジスタ2初期化
	lda #%00000110	; 初期化中はスプライトとBGを表示OFFにする
	sta $2001

	; パレットをロード
	ldx #$00    ; Xレジスタクリア

	; VRAMアドレスレジスタの$2006に、パレットのロード先のアドレス$3F00を指定する。
	lda #$3F
	sta $2006
	lda #$00
	sta $2006

loadPal:			; ラベルは、「ラベル名＋:」の形式で記述
	lda tilepal, x ; Aに(ourpal + x)番地のパレットをロードする

	sta $2007 ; $2007にパレットの値を読み込む

	inx ; Xレジスタに値を1加算している

	cpx #32 ; Xを32(10進数。BGとスプライトのパレットの総数)と比較して同じかどうか比較している	
	bne loadPal ;	上が等しくない場合は、loadpalラベルの位置にジャンプする
	; Xが32ならパレットロード終了

	; 属性(BGのパレット指定データ)をロード

	; $23C0の属性テーブルにロードする
	lda #$23
	sta $2006
	lda #$C0
	sta $2006

	ldx #$00    ; Xレジスタクリア
	lda #%00000000				; ４つともパレット0番
	; 0番か1番にする
loadAttrib
	eor #%01010101				; XOR演算で一つおきのビットを交互に０か１にする
	sta $2007							; $2007に属性の値($0か$55)を読み込む
	; 64回(全キャラクター分)ループする
	inx
	cpx #64
	bne loadAttrib

	; ネームテーブル生成

	; $2000のネームテーブルに生成する
	lda #$20
	sta $2006
	lda #$00
	sta $2006

	lda #$00        ; 0番(真っ黒)
	ldy #$00    ; Yレジスタ初期化
loadNametable1:
	ldx Star_Tbl, y			; Starテーブルの値をXに読み込む
loadNametable2:
	sta $2007				; $2007に属性の値を読み込む
	dex							; X減算
	bne loadNametable2	; まだ0でないならばループして黒を出力する
	; 1番か2番のキャラをYの値から交互に取得
	tya							; Y→A
	and #1					; A AND 1
	adc #1					; Aに1加算して1か2に
	sta $2007				; $2007に属性の値を読み込む
	lda #$00        ; 0番(真っ黒)
	iny							; Y加算
	cpy #20					; 20回(星テーブルの数)ループする
	bne loadNametable1

	; １番目のスプライト座標初期化
	lda X_Pos_Init
	sta Sprite1_X
	lda Y_Pos_Init
	sta Sprite1_Y
	; ２番目のスプライト座標更新サブルーチンをコール
	jsr setSprite2
	; ２番目のスプライトを水平反転
	lda #%01000000
	sta Sprite2_S

	; PPUコントロールレジスタ2初期化
	lda #%00011110	; スプライトとBGの表示をONにする
	sta $2001

	; サウンドレジスタ初期化
	lda #0
	sta $4015

	; PPUコントロールレジスタ1の割り込み許可フラグを立てる
	lda #%10001000
	sta $2000

infinityLoop:					; VBlank割り込み発生を待つだけの無限ループ
	jmp infinityLoop

mainLoop:					; メインループ

	; スプライト描画(DMAを利用)
	lda #$3  ; スプライトデータは$0300番地からなので、3をロードする。
	sta $4014 ; スプライトDMAレジスタにAをストアして、スプライトデータをDMA転送する
	
	; BGスクロール
	lda $2002			; スクロール値クリア
	lda <Scroll_X	; Xのスクロール値をロード
	sta $2005			; X方向スクロール（Y方向は固定)
	inc <Scroll_X	; スクロール値を加算

	; パッドI/Oレジスタの準備
	lda #$01
	sta $4016
	lda #$00
	sta $4016

	; パッド入力チェック
	lda $4016  ; Aボタン
	and #1     ; AND #1
	beq isBKEYdown  ; 0ならばIsBKEYdownへジャンプ
	jsr AKEYSound

isBKEYdown:
	lda $4016  ; Bボタン
	and #1     ; AND #1
	beq isSelectdown  ; 0ならばisSelectdownへジャンプ
	jsr BKEYSound

isSelectdown
	lda $4016  ; Selectボタンをスキップ
	lda $4016  ; Startボタンをスキップ
	lda $4016  ; 上ボタン
	and #1     ; AND #1
	bne UPKEYdown  ; 0でないならば押されてるのでUPKeydownへジャンプ
	
	lda $4016  ; 下ボタン
	and #1     ; AND #1
	bne DOWNKEYdown ; 0でないならば押されてるのでDOWNKeydownへジャンプ

	lda $4016  ; 左ボタン
	and #1     ; AND #1
	bne LEFTKEYdown ; 0でないならば押されてるのでLEFTKeydownへジャンプ

	lda $4016  ; 右ボタン
	and #1     ; AND #1
	bne RIGHTKEYdown ; 0でないならば押されてるのでRIGHTKeydownへジャンプ
	jmp NOTHINGdown  ; なにも押されていないならばNOTHINGdownへ

UPKEYdown:
	dec Sprite1_Y	; Y座標を1減算
	jmp NOTHINGdown

DOWNKEYdown:
	inc Sprite1_Y ; Y座標を1加算
	jmp NOTHINGdown

LEFTKEYdown:
	dec Sprite1_X	; X座標を1減算
	jmp NOTHINGdown 

RIGHTKEYdown:
	inc Sprite1_X	; X座標を1加算
	; この後NOTHINGdownなのでジャンプする必要無し

NOTHINGdown:
	; ２番目のスプライト座標更新サブルーチンをコール
	jsr setSprite2

	; サウンド待ちカウンタ
	lda <Sound_A
	beq dec_B
	dec <Sound_A
dec_B:
	lda <Sound_B
	beq NMIEnd
	dec <Sound_B

NMIEnd:
	rti									; 割り込みから復帰

AKEYSound:
	; サウンド待ちカウンタAが0でない場合はサウンドを鳴らさない
	lda <Sound_A
	beq AKEYSoundSub
	rts
AKEYSoundSub:
	lda #10			; 1/6秒に1回鳴らす
	sta <Sound_A

	lda $4015		; サウンドレジスタ
	ora #%00000001	; 矩形波チャンネル１を有効にする
	sta $4015

	lda #%10111111
	sta $4000		; 矩形波チャンネル１制御レジスタ１

	lda #%10101011
	sta $4001		; 矩形波チャンネル１制御レジスタ２
	lda Sprite1_X		; お遊びでX座標を入れてみる
	sta $4002		; 矩形波チャンネル１周波数レジスタ１

	lda #%11111011
	sta $4003		; 矩形波チャンネル１周波数レジスタ２

	rts

BKEYSound:
	; サウンド待ちカウンタBが0でない場合はサウンドを鳴らさない
	lda <Sound_B
	beq BKEYSoundSub
	rts
BKEYSoundSub:
	lda #10			; 1/6秒に1回鳴らす
	sta <Sound_B

	lda $4015		; サウンドレジスタ
	ora #%00000010	; 矩形波チャンネル２を有効にする
	sta $4015

	lda #%10111111
	sta $4004		; 矩形波チャンネル２制御レジスタ１

	lda #%10000100
	sta $4005		; 矩形波チャンネル２制御レジスタ２
	lda Sprite1_Y		; お遊びでY座標を入れてみる
	sta $4006		; 矩形波チャンネル２周波数レジスタ１

	lda #%11111000
	sta $4007		; 矩形波チャンネル２周波数レジスタ２

	rts

setSprite2:
	; ２番目のスプライトの座標更新サブルーチン
	clc					;　adcの前にキャリーフラグをクリア
	lda Sprite1_X
	adc #8 		; 8ﾄﾞｯﾄ右にずらす
	sta Sprite2_X
	lda Sprite1_Y
	sta Sprite2_Y
	rts

	; 初期データ
X_Pos_Init   .db 20       ; X座標初期値
Y_Pos_Init   .db 40       ; Y座標初期値

	; 星テーブルデータ(20個)
Star_Tbl    .db 60,45,35,60,90,65,45,20,90,10,30,40,65,25,65,35,50,35,40,35

tilepal: .incbin "giko2.pal" ; パレットをincludeする

	.bank 2       ; バンク２
	.org $0000    ; $0000から開始

	.incbin "giko2.bkg"  ; 背景データのバイナリィファイルをincludeする
	.incbin "giko2.spr"  ; スプライトデータのバイナリィファイルをincludeする
