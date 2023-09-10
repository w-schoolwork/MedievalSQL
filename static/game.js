$('.game').blockrain({
	showFieldOnStart: true,
	playText: 'Welcome to brand-neutral block game',
	restartButtonText: 'Try again',
	theme: {
		background: '#040304',
		backgroundGrid: '#000',
		complexBlocks: {
			line: ['/static/blockrain/assets/blocks/custom/line.png', '/static/blockrain/assets/blocks/custom/line.png'],
			square: '/static/blockrain/assets/blocks/custom/square.png',
			arrow: '/static/blockrain/assets/blocks/custom/arrow.png',
			rightHook: '/static/blockrain/assets/blocks/custom/rightHook.png',
			leftHook: '/static/blockrain/assets/blocks/custom/leftHook.png',
			rightZag: '/static/blockrain/assets/blocks/custom/rightZag.png',
			leftZag: '/static/blockrain/assets/blocks/custom/leftZag.png'
		}
	},

	onStart: () => {
		document.querySelector('#ready').disabled = true;
	},

	onGameOver: (score) => {
		document.querySelector('#score').value = Math.max(score, document.querySelector('#score').value);
		document.querySelector('#ready').disabled = false;
	},

	onLine: (lines, scoreIncrement, score) => {
		document.querySelector('#score').value = Math.max(score, document.querySelector('#score').value);
	}
})