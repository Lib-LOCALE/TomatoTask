<script lang="ts">
	/**
	 * Logo TomatoTask : une tomate de majolique peinte au trait de manganèse.
	 * - à l'apparition, le dessin se peint de bas en haut comme un carreau ;
	 * - pendant une session de travail (`active`), la feuille ondule et le fruit respire ;
	 * - au survol, la feuille frémit ;
	 * - aucune animation si l'utilisateur préfère réduire les mouvements.
	 */
	let { size = 30, active = false }: { size?: number; active?: boolean } = $props();
</script>

<svg
	class="brand-logo"
	class:active
	width={size}
	height={size}
	viewBox="0 0 32 32"
	aria-hidden="true"
	focusable="false"
>
	<g class="fruit">
		<path
			class="body"
			d="M16 9.6C9.4 8.6 4.6 12.9 4.6 19.1 4.6 25.4 9.6 29 16 29s11.4-3.6 11.4-9.9c0-6.2-4.8-10.5-11.4-9.5Z"
		/>
		<path class="glint" d="M8.6 16.6c.5-2.3 2-3.7 4-4.1" />
	</g>
	<path class="stem" d="M16 10.5c0-3 .8-5.2 2.4-7" />
	<path
		class="calyx"
		d="M16 9.2 19.8 7.6 18.2 10.6 22 11.7 17.7 12.2 16 14.6 14.3 12.2 10 11.7 13.8 10.6 12.2 7.6Z"
	/>
	<path class="leaf" d="M18 5.4c2.4-2.6 5.6-3 8.2-1.4-1.6 3-4.8 4-8.2 1.4Z" />
</svg>

<style>
	.brand-logo {
		--logo-fruit: #ea6652;
		--logo-leaf: #6cb98a;
		--logo-line: #4a2f27;
		flex: none;
		overflow: visible;
		animation: logo-paint 900ms cubic-bezier(0.3, 0.7, 0.4, 1) both;
	}
	.body {
		fill: var(--logo-fruit);
		stroke: var(--logo-line);
		stroke-width: 1.1;
	}
	.glint {
		fill: none;
		stroke: #fff;
		stroke-width: 1.4;
		stroke-linecap: round;
		opacity: 0.75;
	}
	.stem {
		fill: none;
		stroke: var(--logo-leaf);
		stroke-width: 1.8;
		stroke-linecap: round;
	}
	.calyx,
	.leaf {
		fill: var(--logo-leaf);
		stroke: var(--logo-line);
		stroke-width: 0.8;
		stroke-linejoin: round;
	}
	/* Pivots exprimés dans le repère du viewBox : la feuille tourne autour de sa base */
	.leaf {
		transform-origin: 18px 5.4px;
	}
	.fruit {
		transform-origin: 16px 29px;
	}
	.brand-logo:hover .leaf {
		animation: leaf-rustle 700ms ease-in-out;
	}
	.active .leaf {
		animation: leaf-sway 4s ease-in-out infinite;
	}
	.active .fruit {
		animation: fruit-breathe 4s ease-in-out infinite;
	}
	@keyframes logo-paint {
		from {
			clip-path: inset(100% 0 0);
		}
		to {
			clip-path: inset(-10% -10% -10% -10%);
		}
	}
	@keyframes leaf-sway {
		0%,
		100% {
			transform: rotate(0deg);
		}
		50% {
			transform: rotate(-12deg);
		}
	}
	@keyframes leaf-rustle {
		0%,
		100% {
			transform: rotate(0deg);
		}
		30% {
			transform: rotate(-14deg);
		}
		65% {
			transform: rotate(6deg);
		}
	}
	@keyframes fruit-breathe {
		0%,
		100% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.035, 1.02);
		}
	}
	@media (prefers-reduced-motion: reduce) {
		.brand-logo,
		.brand-logo .leaf,
		.brand-logo .fruit {
			animation: none !important;
		}
	}
</style>
