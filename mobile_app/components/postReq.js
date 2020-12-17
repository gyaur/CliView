import jsonServer from '../api/jsonServer';

export default function postReq(url, data) {
	jsonServer.post(url, data)
		.then(function (response) {
			console.log(response);
			return { status: 200 }
		})
		.catch(function (error) {
			console.log(error);
			return { status: 400 }
		});
}
