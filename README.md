# pam tester - a tool to help test pam

This allows you to "poke" at pam and it's various phases to test your authentication stacks are
working as you expect.

	# Wrong password
	# pam_tester sudo william
	Enter password:
	2022-11-14T04:01:14.390159Z ERROR Authentication failed e=AUTH_ERR
	2022-11-14T04:01:14.390247Z ERROR Session failed e=PERM_DENIED

	# Right password
	# pam_tester sudo william
	Enter password:
	2022-11-14T04:01:22.351713Z  INFO Successfully authenticated!
	2022-11-14T04:01:22.358741Z  INFO Successfully opened session!

