import React, { useState } from 'react';
import { View, TouchableOpacity, StyleSheet } from 'react-native';
import Slider from '@react-native-community/slider';
import { FontAwesome5, FontAwesome } from '@expo/vector-icons';

import jsonServer from '../api/jsonServer'

class VolumeSlide extends React.Component {

  constructor() {
    super();
    this.state = { sliderValue: 50 }
  }


  setVolReq(url, data) {
    
    jsonServer.post(url, data)
      .then(function (response) {
        console.log(response);
      })
      .catch(function (error) {
        console.log(error);
        return { status: 400 }
      });
      return { status: 200 }
  }

  render() {
    return (
      <View style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "center",
        alignItems: "center",
        width: "90%"
      }}>
        <TouchableOpacity
          style={{
            flexGrow: 1,
            marginHorizontal: 10,
            padding: 10,
          }}
          onPress={() => {
            if (this.state.sliderValue === 0) {
              this.setState({sliderValue:70});
              this.setVolReq('/volume', { "volume": 7 })
            }
            else {
              this.setState({sliderValue:0});
              this.setVolReq('/volume', { "volume": 0 })
            }
          }} >
          {this.state.sliderValue === 0 ?
            <FontAwesome5 name="volume-mute" size={24} color="#c8d6e5" /> :
            <FontAwesome name="volume-up" size={24} color="#c8d6e5" />
          }

        </TouchableOpacity>

        <Slider
          style={{
            flexGrow: 1,
            width: 250,
            height: 40,
          }}
          minimumValue={0}
          maximumValue={100}
          minimumTrackTintColor="#c8d6e5"
          maximumTrackTintColor="#576574"
          thumbTintColor="#c8d6e5"
          step={10}
          value={this.state.sliderValue}
          onValueChange={
            (sliderValue) => this.setState({sliderValue})
          }
          onSlidingComplete={() => {
            //console.log('volume should be updated to :', sliderValue) 
            this.setVolReq('/volume', { "volume": this.state.sliderValue / 10 })
          }}
        />

      </View>
    )
  }
};

export default VolumeSlide;