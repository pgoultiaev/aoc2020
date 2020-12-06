run-day-x:
	rustc day$(x).rs && ./day$(x) && rm day$(x)