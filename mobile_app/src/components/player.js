import React from 'react';
import { View } from 'react-native';

import Backward from './player/backward'
import PlayPause from './player/play_pause'
import Forward from './player/forward'

const Player = ({ newIP }) => {

  return (
    <View style={{
      //display: "block",
      marginLeft: "auto",
      marginRight: "auto",
      marginTop: 50,
      flexDirection: "row",
    }}>
      <Backward IP={newIP} />
      <PlayPause IP={newIP} />
      <Forward IP={newIP} />
    </View>
  )
};



export default Player;