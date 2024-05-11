/*
 * Copyright (c) 2024, MLC 'Strawmelonjuice' Bloeiman
 *
 * Licensed under the BSD 3-Clause License. See the LICENSE file for more info.
 */
document.forms["signup"]["username"].placeholder = funnyRandomUserName();
setTimeout(() => {
	document.forms["signup"]["email"].placeholder = `${funnyRandomUserName()}@${
		fejson.instance.config.interinstance.iid
	}`;
}, 200);
function checkusername() {
	const inp = document.forms["signup"]["username"];
	axios
		.post("/api/fe/auth-create/check-username", {
			u: document.forms["signup"]["username"].value,
		})
		.then((resp) => {
			if (resp.data.Ok === true) {
				document.getElementById("usernameLabel").innerHTML =
					`Username&emsp;&emsp;<span class="text-green-200">⬅ Username is available!&nbsp;<img src="/green-check.svg" class="inline"></span>`;
			} else {
				switch (resp.data.Why) {
					case "InvalidChars":
						{
							document.getElementById("usernameLabel").innerHTML =
								`Username&emsp;&emsp;<span class="text-red-200">⬅ There are characters in this username that are not allowed!&nbsp;<img src="/red-cross.svg" class="inline"></span>`;
						}
						break;
					case "TooShort":
						{
							document.getElementById("usernameLabel").innerHTML =
								`Username&emsp;&emsp;<span class="text-red-200">⬅ That username is too short!&nbsp;<img src="/red-cross.svg" class="inline"></span>`;
						}
						break;
					case "userExists":
						{
							document.getElementById("usernameLabel").innerHTML =
								`Username&emsp;&emsp;<span class="text-red-200">⬅ Someone already claimed that username!&nbsp;<img src="/red-cross.svg" class="inline"></span>`;
						}
						break;
					default: {
						document.getElementById("usernameLabel").innerHTML =
							`Username&emsp;&emsp;<span class="text-red-200">⬅ Username is not available!&nbsp;<img src="/red-cross.svg" class="inline"></span>`;
					}
				}
			}
		})
		.catch((error) => {
			console.log(error);
			document.getElementById("usernameLabel").innerHTML =
				`Username&emsp;&emsp;⬅ Error while checking this username!`;
		});
}
document.forms["signup"]["username"].setAttribute("oninput", "checkusername()");
const submitbutton = document.forms["signup"]["submitbutton"];
function d(a) {
	if (a.data.Ok === true) {
		submitbutton.innerHTML = `<div style="background-image: url('/green-check.svg'); background-repeat: no-repeat; background-size: cover;" class="pl-max pr-max relative h-10 w-10"></div>`;
		document.getElementById("Aaa1").innerText =
			"Sign-up successful! You will be forwarded now.";
		setTimeout(() => {
			window.location.assign("/home/");
		}, 3000);
	} else {
		submitbutton.innerHTML = `<div style="background-image: url('/red-cross.svg'); background-repeat: no-repeat; background-size: cover;" class="pl-max pr-max relative h-10 w-10"></div>`;
		document.getElementById("Aaa1").innerText = a.data.Errorvalue;
		setTimeout(() => {
			submitbutton.innerText = "Sign up";
			submitbutton.removeAttribute("disabled");
		}, 3000);
	}
}

function regstry() {
	submitbutton.innerHTML = `<div style="background-image: url('/spinner.svg'); background-repeat: no-repeat; background-size: cover;" class="pl-max pr-max relative h-10 w-10"></div>`;
	submitbutton.setAttribute("disabled", "");
	setTimeout(() => {
		submitbutton.innerText = "Retry";
		submitbutton.removeAttribute("disabled");
	}, 9600);
	document.getElementById("Aaa1").innerText = "Creating account...";

	// timeout to allow spinner to show up
	setTimeout(() => {
		axios
			.post("/api/fe/auth-create", {
				username: document.forms["signup"]["username"].value,
				email: document.forms["signup"]["email"].value,
				password: document.forms["signup"]["password"].value,
			})
			.then(d)
			.catch((error) => {
				console.log(error);
			});
	}, 500);
}
