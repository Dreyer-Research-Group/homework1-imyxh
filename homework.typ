#import "@preview/physica:0.9.5": *

#import "@preview/fancy-units:0.1.1": (num, unit, qty,
	fancy-units-configure, add-macros,
)

#let bodysize = 10pt
#let codesize = 8pt

#let homework(title: none, date: none, name: none, email: none, doc,) = {
	set page(
		paper: "us-letter",
		header: context [
			#set text(luma(50%))
			#name #h(1fr)
			#email #h(1fr)
			page #here().page() of #counter(page).final().at(0)
		],
	)

	show math.equation: set text(font: "Libertinus Math")
	show math.equation: set block(spacing: 1.6em)
	show math.equation.where(block: true): set align(left)
	show math.equation.where(block: true): pad.with(left: 1cm)

	show raw: set text(font: "Iosevka", size: codesize)
	show raw.where(block: true): set par(leading: 0.4em)
	show raw.where(block: true): it => pad(left: 16mm, it)

	fancy-units-configure(
		quantity-separator: sym.space.nobreak,
	)

	set par(
		justify: true,
		spacing: 1.6em,
		leading: 0.85em,
	)

	set text(
		font: "Libertinus Serif",
		size: bodysize,
	)

	text(18pt, title)
	h(1fr)
	date.display()

	v(4mm)

	set align(left)
	doc
}

#let problem(body, number: none) = {
	block(
		width: 100%,
		fill: luma(95%),
		inset: 4mm,
		radius: 0mm,
		[
			#context place(
				dx: -measure(number).width - 6mm,
				text(weight: "bold")[#number],
			)
			#body
		],
	)
}

#let subproblem(body, number: none) = {
	context place(
		dx: -measure(number).width - 6mm,
		text(weight: "bold")[#number],
	)
	body
}

#let anscolor = rgb("#990000")
#let ans(body) = {
	set text(fill: anscolor)
	body
}

// TODO: abbreviate difference()

#let implies = {[#math.quad #sym.arrow.r.double #math.quad]}
#let implied = {[#math.quad #sym.arrow.l.double #math.quad]}

#let repby = symbol("≗")

#let qed = {h(1fr); symbol("■")}
#let miniqed = {h(1fr); symbol("□")}

#let ev = expval

// variant greek letters
#let ve = sym.epsilon.alt
#let vp = sym.phi.alt

