import React from 'react';
import { View, TouchableOpacity } from 'react-native';
import { AntDesign } from '@expo/vector-icons';
import axios from 'axios'


class BigBackward extends React.Component {

	constructor() {
		super();
		this.state = {}
	}


	backwardReq() {
		const res = axios.post(this.props.IP + '/seek ', { "ammount": -90 })
			.then(function (response) {
				console.log(response);
			})
			.catch(function (error) {
				console.log(error);
				return ({ status: 400 })
			})
		return ({ status: 200 })

	}

	render() {
		return (
			<View>
				<TouchableOpacity
					style={{
						alignItems: "center",
						paddingTop: 15,
						margin: 15,
						marginLeft: 25,
						flex: 1,
						borderRadius: 50
					}}
					onPress={() => { this.backwardReq() }}
				>
					<AntDesign name="banckward" size={24} color="#c8d6e5" />
				</TouchableOpacity>
			</View >
		)
	}

}


export default BigBackward;

