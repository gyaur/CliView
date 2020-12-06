import React, { useState } from 'react';
import { View, StyleSheet } from 'react-native';

import Backward from './player/backward'
import PlayPause from './player/play_pause'
import Forward from './player/forward'

const Player = () => {

  return (
    <View style={{
      //display: "block",
      marginLeft: "auto",
      marginRight: "auto",
      marginTop: 50,
      flexDirection: "row",
    }}>
      <Backward />
      <PlayPause />
      <Forward />
    </View>
  )
};



export default Player;