import React from 'react';
import { View } from 'react-native';

import Backward from './player/backward'
import PlayPause from './player/play_pause'
import Forward from './player/forward'
import BigBackward from './player/bigBackword';
import BigForward from './player/bigForward';

const Player = ({ newIP }) => {

  return (
    <View style={{
      //display: "block",
      marginLeft: "auto",
      marginRight: "auto",
      marginTop: 30,
      flexDirection: "row",
    }}>
      <BigBackward IP={newIP} />
      <Backward IP={newIP} />
      <PlayPause IP={newIP} />
      <Forward IP={newIP} />
      <BigForward IP={newIP} />
    </View>
  )
};



export default Player;