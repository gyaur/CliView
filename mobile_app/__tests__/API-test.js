import "react-native";
import React from "react";
import renderer from 'react-test-renderer'

import PlayPause from '../components/player/play_pause'
import Backward from "../components/player/backward"
import Forward from "../components/player/forward"
import VolumeSlide from "../components/volumeSlide";



it("30 sec backwords", async () => {
  let backwards = renderer.create(<Backward/>).getInstance();
  expect(backwards.backwardReq()).toEqual({ status: 200 })
})


it("play/pause", async () => {
  let playpause = renderer.create(<PlayPause/>).getInstance();
  expect(playpause.pauseReq()).toEqual({ status: 200 })
  expect(playpause.playReq()).toEqual({ status: 200 })
})

it("setting the volume", async () => {
  let volumeSlide = renderer.create(<VolumeSlide/>).getInstance();
  expect(volumeSlide.setVolReq("/volume", { "volume": 7 })).toEqual({ status: 200 })
  expect(volumeSlide.setVolReq("/volume", { "volume": 0 })).toEqual({ status: 200 })
  expect(volumeSlide.setVolReq("/volume", { "volume": 10 })).toEqual({ status: 200 })
})

it("30 sec forwards", async () => {
  let forward = renderer.create(<Forward/>).getInstance();
  expect(forward.forwardreq()).toEqual({ status: 200 })
})
