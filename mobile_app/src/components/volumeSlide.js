import React, { useState } from 'react';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';
import Slider from '@react-native-community/slider';
import { FontAwesome5, FontAwesome } from '@expo/vector-icons';

const VolumeSlide = () => {

  const [sliderValue, setSliderValue] = useState(15);

  return (
    <View style={styles.slideContainerStyle}>
      <TouchableOpacity
        style={styles.iconStyle}
        onPress={() => { setSliderValue(0) }} >
        {sliderValue === 0 ?
          <FontAwesome5 name="volume-mute" size={24} color="#c8d6e5" /> :
          <FontAwesome name="volume-up" size={24} color="#c8d6e5" />
        }

      </TouchableOpacity>

      <Slider
        style={styles.slideStyle}
        minimumValue={0}
        maximumValue={100}
        minimumTrackTintColor="#c8d6e5"
        maximumTrackTintColor="#576574"
        thumbTintColor="#c8d6e5"
        step={1}
        value={sliderValue}
        onValueChange={
          (sliderValue) => setSliderValue(sliderValue)
        }
        onSlidingComplete={() => { console.log('volume should be updated to :', sliderValue) }}
      />


    </View>
  )
};

const styles = StyleSheet.create({
  slideContainerStyle: {
    display: "flex",
    flexDirection: "row",
    justifyContent: "center",
    alignItems: "center",
    width: "90%"
  },
  slideStyle: {
    flexGrow: 1,
    width: 250,
    height: 40,
  },
  iconStyle: {
    flexGrow: 1,
    marginHorizontal: 10,
    padding: 10,
  }
});

export default VolumeSlide;