use crate::messages::*;
use prost::Name;
///Generated list of DFHack stubs. Each stub communicates with a plugin.
pub struct Stubs<TChannel: crate::Channel> {
    channel: TChannel,
}
impl<TChannel: crate::Channel> From<TChannel> for Stubs<TChannel> {
    ///Initialize all the generated stubs.
    fn from(channel: TChannel) -> Self {
        Self { channel }
    }
}
impl<'a, TChannel: crate::Channel> Stubs<TChannel> {
    ///RPCs of the  plugin
    pub fn core(&'a mut self) -> crate::stubs::Core<'a, TChannel> {
        crate::stubs::Core::new(&mut self.channel)
    }
    ///RPCs of the RemoteFortressReader plugin
    pub fn remote_fortress_reader(
        &'a mut self,
    ) -> crate::stubs::RemoteFortressReader<'a, TChannel> {
        crate::stubs::RemoteFortressReader::new(&mut self.channel)
    }
    ///RPCs of the mypluginname plugin
    pub fn mypluginname(&'a mut self) -> crate::stubs::Mypluginname<'a, TChannel> {
        crate::stubs::Mypluginname::new(&mut self.channel)
    }
}
#[cfg(feature = "reflection")]
impl<TChannel: crate::Channel> crate::reflection::StubReflection for Stubs<TChannel> {
    fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
        let mut methods = Vec::new();
        methods.extend(Core::<TChannel>::list_methods());
        methods.extend(Core::<TChannel>::list_methods());
        methods.extend(Core::<TChannel>::list_methods());
        methods
    }
}
///RPC for the "" plugin.
pub struct Core<'a, TChannel: crate::Channel> {
    ///Reference to the client to exchange messages.
    pub channel: &'a mut TChannel,
}
impl<'a, TChannel: crate::Channel> Core<'a, TChannel> {
    ///Initialize the plugin from a channel to DFHack.
    pub fn new(channel: &'a mut TChannel) -> Self {
        Self { channel }
    }
    ///Method `BindMethod` from the plugin ``
    pub fn bind_method(
        &mut self,
        request: CoreBindRequest,
    ) -> Result<crate::Reply<CoreBindReply>, TChannel::TError> {
        let _response: crate::Reply<CoreBindReply> =
            self.channel.request("", "BindMethod", request)?;
        Ok(_response)
    }
    ///Method `CoreResume` from the plugin ``
    pub fn core_resume(&mut self) -> Result<crate::Reply<i32>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<IntMessage> =
            self.channel.request("", "CoreResume", request)?;
        let _response = crate::Reply {
            reply: _response.value,
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `CoreSuspend` from the plugin ``
    pub fn core_suspend(&mut self) -> Result<crate::Reply<i32>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<IntMessage> =
            self.channel.request("", "CoreSuspend", request)?;
        let _response = crate::Reply {
            reply: _response.value,
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `GetDFVersion` from the plugin ``
    pub fn get_df_version(&mut self) -> Result<crate::Reply<String>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<StringMessage> =
            self.channel.request("", "GetDFVersion", request)?;
        let _response = crate::Reply {
            reply: _response.value.clone(),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `GetVersion` from the plugin ``
    pub fn get_version(&mut self) -> Result<crate::Reply<String>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<StringMessage> =
            self.channel.request("", "GetVersion", request)?;
        let _response = crate::Reply {
            reply: _response.value.clone(),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `GetWorldInfo` from the plugin ``
    pub fn get_world_info(&mut self) -> Result<crate::Reply<GetWorldInfoOut>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<GetWorldInfoOut> =
            self.channel.request("", "GetWorldInfo", request)?;
        Ok(_response)
    }
    ///Method `ListEnums` from the plugin ``
    pub fn list_enums(&mut self) -> Result<crate::Reply<ListEnumsOut>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<ListEnumsOut> =
            self.channel.request("", "ListEnums", request)?;
        Ok(_response)
    }
    ///Method `ListJobSkills` from the plugin ``
    pub fn list_job_skills(&mut self) -> Result<crate::Reply<ListJobSkillsOut>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<ListJobSkillsOut> =
            self.channel.request("", "ListJobSkills", request)?;
        Ok(_response)
    }
    ///Method `ListMaterials` from the plugin ``
    pub fn list_materials(
        &mut self,
        request: ListMaterialsIn,
    ) -> Result<crate::Reply<ListMaterialsOut>, TChannel::TError> {
        let _response: crate::Reply<ListMaterialsOut> =
            self.channel.request("", "ListMaterials", request)?;
        Ok(_response)
    }
    ///Method `ListSquads` from the plugin ``
    pub fn list_squads(
        &mut self,
        request: ListSquadsIn,
    ) -> Result<crate::Reply<ListSquadsOut>, TChannel::TError> {
        let _response: crate::Reply<ListSquadsOut> =
            self.channel.request("", "ListSquads", request)?;
        Ok(_response)
    }
    ///Method `ListUnits` from the plugin ``
    pub fn list_units(
        &mut self,
        request: ListUnitsIn,
    ) -> Result<crate::Reply<ListUnitsOut>, TChannel::TError> {
        let _response: crate::Reply<ListUnitsOut> =
            self.channel.request("", "ListUnits", request)?;
        Ok(_response)
    }
    ///Method `RunCommand` from the plugin ``
    pub fn run_command(
        &mut self,
        request: CoreRunCommandRequest,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel.request("", "RunCommand", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `RunLua` from the plugin ``
    pub fn run_lua(
        &mut self,
        request: CoreRunLuaRequest,
    ) -> Result<crate::Reply<StringListMessage>, TChannel::TError> {
        let _response: crate::Reply<StringListMessage> =
            self.channel.request("", "RunLua", request)?;
        Ok(_response)
    }
    ///Method `SetUnitLabors` from the plugin ``
    pub fn set_unit_labors(
        &mut self,
        request: SetUnitLaborsIn,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel.request("", "SetUnitLabors", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
}
#[cfg(feature = "reflection")]
impl<TChannel: crate::Channel> crate::reflection::StubReflection for Core<'_, TChannel> {
    fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
        vec![
            crate::reflection::RemoteProcedureDescriptor {
                name: "BindMethod",
                plugin_name: "",
                input_type: CoreBindRequest::full_name(),
                output_type: CoreBindReply::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "CoreResume",
                plugin_name: "",
                input_type: EmptyMessage::full_name(),
                output_type: IntMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "CoreSuspend",
                plugin_name: "",
                input_type: EmptyMessage::full_name(),
                output_type: IntMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetDFVersion",
                plugin_name: "",
                input_type: EmptyMessage::full_name(),
                output_type: StringMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetVersion",
                plugin_name: "",
                input_type: EmptyMessage::full_name(),
                output_type: StringMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetWorldInfo",
                plugin_name: "",
                input_type: EmptyMessage::full_name(),
                output_type: GetWorldInfoOut::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "ListEnums",
                plugin_name: "",
                input_type: EmptyMessage::full_name(),
                output_type: ListEnumsOut::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "ListJobSkills",
                plugin_name: "",
                input_type: EmptyMessage::full_name(),
                output_type: ListJobSkillsOut::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "ListMaterials",
                plugin_name: "",
                input_type: ListMaterialsIn::full_name(),
                output_type: ListMaterialsOut::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "ListSquads",
                plugin_name: "",
                input_type: ListSquadsIn::full_name(),
                output_type: ListSquadsOut::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "ListUnits",
                plugin_name: "",
                input_type: ListUnitsIn::full_name(),
                output_type: ListUnitsOut::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "RunCommand",
                plugin_name: "",
                input_type: CoreRunCommandRequest::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "RunLua",
                plugin_name: "",
                input_type: CoreRunLuaRequest::full_name(),
                output_type: StringListMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "SetUnitLabors",
                plugin_name: "",
                input_type: SetUnitLaborsIn::full_name(),
                output_type: EmptyMessage::full_name(),
            },
        ]
    }
}
///RPC for the "RemoteFortressReader" plugin.
pub struct RemoteFortressReader<'a, TChannel: crate::Channel> {
    ///Reference to the client to exchange messages.
    pub channel: &'a mut TChannel,
}
impl<'a, TChannel: crate::Channel> RemoteFortressReader<'a, TChannel> {
    ///Initialize the plugin from a channel to DFHack.
    pub fn new(channel: &'a mut TChannel) -> Self {
        Self { channel }
    }
    ///Method `CheckHashes` from the plugin `RemoteFortressReader`
    pub fn check_hashes(&mut self) -> Result<crate::Reply<()>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("RemoteFortressReader", "CheckHashes", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `CopyScreen` from the plugin `RemoteFortressReader`
    pub fn copy_screen(&mut self) -> Result<crate::Reply<ScreenCapture>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<ScreenCapture> =
            self.channel
                .request("RemoteFortressReader", "CopyScreen", request)?;
        Ok(_response)
    }
    ///Method `GetBlockList` from the plugin `RemoteFortressReader`
    pub fn get_block_list(
        &mut self,
        request: BlockRequest,
    ) -> Result<crate::Reply<BlockList>, TChannel::TError> {
        let _response: crate::Reply<BlockList> =
            self.channel
                .request("RemoteFortressReader", "GetBlockList", request)?;
        Ok(_response)
    }
    ///Method `GetBuildingDefList` from the plugin `RemoteFortressReader`
    pub fn get_building_def_list(
        &mut self,
    ) -> Result<crate::Reply<BuildingList>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<BuildingList> =
            self.channel
                .request("RemoteFortressReader", "GetBuildingDefList", request)?;
        Ok(_response)
    }
    ///Method `GetCreatureRaws` from the plugin `RemoteFortressReader`
    pub fn get_creature_raws(&mut self) -> Result<crate::Reply<CreatureRawList>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<CreatureRawList> =
            self.channel
                .request("RemoteFortressReader", "GetCreatureRaws", request)?;
        Ok(_response)
    }
    ///Method `GetGameValidity` from the plugin `RemoteFortressReader`
    pub fn get_game_validity(&mut self) -> Result<crate::Reply<bool>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<SingleBool> =
            self.channel
                .request("RemoteFortressReader", "GetGameValidity", request)?;
        let _response = crate::Reply {
            reply: _response.value(),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `GetGrowthList` from the plugin `RemoteFortressReader`
    pub fn get_growth_list(&mut self) -> Result<crate::Reply<MaterialList>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<MaterialList> =
            self.channel
                .request("RemoteFortressReader", "GetGrowthList", request)?;
        Ok(_response)
    }
    ///Method `GetItemList` from the plugin `RemoteFortressReader`
    pub fn get_item_list(&mut self) -> Result<crate::Reply<MaterialList>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<MaterialList> =
            self.channel
                .request("RemoteFortressReader", "GetItemList", request)?;
        Ok(_response)
    }
    ///Method `GetLanguage` from the plugin `RemoteFortressReader`
    pub fn get_language(&mut self) -> Result<crate::Reply<Language>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<Language> =
            self.channel
                .request("RemoteFortressReader", "GetLanguage", request)?;
        Ok(_response)
    }
    ///Method `GetMapInfo` from the plugin `RemoteFortressReader`
    pub fn get_map_info(&mut self) -> Result<crate::Reply<MapInfo>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<MapInfo> =
            self.channel
                .request("RemoteFortressReader", "GetMapInfo", request)?;
        Ok(_response)
    }
    ///Method `GetMaterialList` from the plugin `RemoteFortressReader`
    pub fn get_material_list(&mut self) -> Result<crate::Reply<MaterialList>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<MaterialList> =
            self.channel
                .request("RemoteFortressReader", "GetMaterialList", request)?;
        Ok(_response)
    }
    ///Method `GetPartialCreatureRaws` from the plugin `RemoteFortressReader`
    pub fn get_partial_creature_raws(
        &mut self,
        request: ListRequest,
    ) -> Result<crate::Reply<CreatureRawList>, TChannel::TError> {
        let _response: crate::Reply<CreatureRawList> =
            self.channel
                .request("RemoteFortressReader", "GetPartialCreatureRaws", request)?;
        Ok(_response)
    }
    ///Method `GetPartialPlantRaws` from the plugin `RemoteFortressReader`
    pub fn get_partial_plant_raws(
        &mut self,
        request: ListRequest,
    ) -> Result<crate::Reply<PlantRawList>, TChannel::TError> {
        let _response: crate::Reply<PlantRawList> =
            self.channel
                .request("RemoteFortressReader", "GetPartialPlantRaws", request)?;
        Ok(_response)
    }
    ///Method `GetPauseState` from the plugin `RemoteFortressReader`
    pub fn get_pause_state(&mut self) -> Result<crate::Reply<bool>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<SingleBool> =
            self.channel
                .request("RemoteFortressReader", "GetPauseState", request)?;
        let _response = crate::Reply {
            reply: _response.value(),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `GetPlantList` from the plugin `RemoteFortressReader`
    pub fn get_plant_list(
        &mut self,
        request: BlockRequest,
    ) -> Result<crate::Reply<PlantList>, TChannel::TError> {
        let _response: crate::Reply<PlantList> =
            self.channel
                .request("RemoteFortressReader", "GetPlantList", request)?;
        Ok(_response)
    }
    ///Method `GetPlantRaws` from the plugin `RemoteFortressReader`
    pub fn get_plant_raws(&mut self) -> Result<crate::Reply<PlantRawList>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<PlantRawList> =
            self.channel
                .request("RemoteFortressReader", "GetPlantRaws", request)?;
        Ok(_response)
    }
    ///Method `GetRegionMaps` from the plugin `RemoteFortressReader`
    pub fn get_region_maps(&mut self) -> Result<crate::Reply<RegionMaps>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<RegionMaps> =
            self.channel
                .request("RemoteFortressReader", "GetRegionMaps", request)?;
        Ok(_response)
    }
    ///Method `GetRegionMapsNew` from the plugin `RemoteFortressReader`
    pub fn get_region_maps_new(&mut self) -> Result<crate::Reply<RegionMaps>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<RegionMaps> =
            self.channel
                .request("RemoteFortressReader", "GetRegionMapsNew", request)?;
        Ok(_response)
    }
    ///Method `GetReports` from the plugin `RemoteFortressReader`
    pub fn get_reports(&mut self) -> Result<crate::Reply<Status>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<Status> =
            self.channel
                .request("RemoteFortressReader", "GetReports", request)?;
        Ok(_response)
    }
    ///Method `GetSideMenu` from the plugin `RemoteFortressReader`
    pub fn get_side_menu(&mut self) -> Result<crate::Reply<SidebarState>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<SidebarState> =
            self.channel
                .request("RemoteFortressReader", "GetSideMenu", request)?;
        Ok(_response)
    }
    ///Method `GetTiletypeList` from the plugin `RemoteFortressReader`
    pub fn get_tiletype_list(&mut self) -> Result<crate::Reply<TiletypeList>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<TiletypeList> =
            self.channel
                .request("RemoteFortressReader", "GetTiletypeList", request)?;
        Ok(_response)
    }
    ///Method `GetUnitList` from the plugin `RemoteFortressReader`
    pub fn get_unit_list(&mut self) -> Result<crate::Reply<UnitList>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<UnitList> =
            self.channel
                .request("RemoteFortressReader", "GetUnitList", request)?;
        Ok(_response)
    }
    ///Method `GetUnitListInside` from the plugin `RemoteFortressReader`
    pub fn get_unit_list_inside(
        &mut self,
        request: BlockRequest,
    ) -> Result<crate::Reply<UnitList>, TChannel::TError> {
        let _response: crate::Reply<UnitList> =
            self.channel
                .request("RemoteFortressReader", "GetUnitListInside", request)?;
        Ok(_response)
    }
    ///Method `GetVersionInfo` from the plugin `RemoteFortressReader`
    pub fn get_version_info(&mut self) -> Result<crate::Reply<VersionInfo>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<VersionInfo> =
            self.channel
                .request("RemoteFortressReader", "GetVersionInfo", request)?;
        Ok(_response)
    }
    ///Method `GetViewInfo` from the plugin `RemoteFortressReader`
    pub fn get_view_info(&mut self) -> Result<crate::Reply<ViewInfo>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<ViewInfo> =
            self.channel
                .request("RemoteFortressReader", "GetViewInfo", request)?;
        Ok(_response)
    }
    ///Method `GetWorldMap` from the plugin `RemoteFortressReader`
    pub fn get_world_map(&mut self) -> Result<crate::Reply<WorldMap>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<WorldMap> =
            self.channel
                .request("RemoteFortressReader", "GetWorldMap", request)?;
        Ok(_response)
    }
    ///Method `GetWorldMapCenter` from the plugin `RemoteFortressReader`
    pub fn get_world_map_center(&mut self) -> Result<crate::Reply<WorldMap>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<WorldMap> =
            self.channel
                .request("RemoteFortressReader", "GetWorldMapCenter", request)?;
        Ok(_response)
    }
    ///Method `GetWorldMapNew` from the plugin `RemoteFortressReader`
    pub fn get_world_map_new(&mut self) -> Result<crate::Reply<WorldMap>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<WorldMap> =
            self.channel
                .request("RemoteFortressReader", "GetWorldMapNew", request)?;
        Ok(_response)
    }
    ///Method `JumpCommand` from the plugin `RemoteFortressReader`
    pub fn jump_command(
        &mut self,
        request: MoveCommandParams,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("RemoteFortressReader", "JumpCommand", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `MenuQuery` from the plugin `RemoteFortressReader`
    pub fn menu_query(&mut self) -> Result<crate::Reply<MenuContents>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<MenuContents> =
            self.channel
                .request("RemoteFortressReader", "MenuQuery", request)?;
        Ok(_response)
    }
    ///Method `MiscMoveCommand` from the plugin `RemoteFortressReader`
    pub fn misc_move_command(
        &mut self,
        request: MiscMoveParams,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("RemoteFortressReader", "MiscMoveCommand", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `MoveCommand` from the plugin `RemoteFortressReader`
    pub fn move_command(
        &mut self,
        request: MoveCommandParams,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("RemoteFortressReader", "MoveCommand", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `MovementSelectCommand` from the plugin `RemoteFortressReader`
    pub fn movement_select_command(
        &mut self,
        value: i32,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let request = IntMessage { value };
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("RemoteFortressReader", "MovementSelectCommand", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `PassKeyboardEvent` from the plugin `RemoteFortressReader`
    pub fn pass_keyboard_event(
        &mut self,
        request: KeyboardEvent,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("RemoteFortressReader", "PassKeyboardEvent", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `ResetMapHashes` from the plugin `RemoteFortressReader`
    pub fn reset_map_hashes(&mut self) -> Result<crate::Reply<()>, TChannel::TError> {
        let request = EmptyMessage::default();
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("RemoteFortressReader", "ResetMapHashes", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `SendDigCommand` from the plugin `RemoteFortressReader`
    pub fn send_dig_command(
        &mut self,
        request: DigCommand,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("RemoteFortressReader", "SendDigCommand", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `SetPauseState` from the plugin `RemoteFortressReader`
    pub fn set_pause_state(&mut self, value: bool) -> Result<crate::Reply<()>, TChannel::TError> {
        let request = SingleBool { value: Some(value) };
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("RemoteFortressReader", "SetPauseState", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `SetSideMenu` from the plugin `RemoteFortressReader`
    pub fn set_side_menu(
        &mut self,
        request: SidebarCommand,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("RemoteFortressReader", "SetSideMenu", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
}
#[cfg(feature = "reflection")]
impl<TChannel: crate::Channel> crate::reflection::StubReflection
    for RemoteFortressReader<'_, TChannel>
{
    fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
        vec![
            crate::reflection::RemoteProcedureDescriptor {
                name: "CheckHashes",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "CopyScreen",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: ScreenCapture::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetBlockList",
                plugin_name: "RemoteFortressReader",
                input_type: BlockRequest::full_name(),
                output_type: BlockList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetBuildingDefList",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: BuildingList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetCreatureRaws",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: CreatureRawList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetGameValidity",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: SingleBool::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetGrowthList",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: MaterialList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetItemList",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: MaterialList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetLanguage",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: Language::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetMapInfo",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: MapInfo::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetMaterialList",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: MaterialList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetPartialCreatureRaws",
                plugin_name: "RemoteFortressReader",
                input_type: ListRequest::full_name(),
                output_type: CreatureRawList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetPartialPlantRaws",
                plugin_name: "RemoteFortressReader",
                input_type: ListRequest::full_name(),
                output_type: PlantRawList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetPauseState",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: SingleBool::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetPlantList",
                plugin_name: "RemoteFortressReader",
                input_type: BlockRequest::full_name(),
                output_type: PlantList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetPlantRaws",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: PlantRawList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetRegionMaps",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: RegionMaps::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetRegionMapsNew",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: RegionMaps::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetReports",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: Status::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetSideMenu",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: SidebarState::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetTiletypeList",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: TiletypeList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetUnitList",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: UnitList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetUnitListInside",
                plugin_name: "RemoteFortressReader",
                input_type: BlockRequest::full_name(),
                output_type: UnitList::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetVersionInfo",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: VersionInfo::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetViewInfo",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: ViewInfo::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetWorldMap",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: WorldMap::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetWorldMapCenter",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: WorldMap::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "GetWorldMapNew",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: WorldMap::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "JumpCommand",
                plugin_name: "RemoteFortressReader",
                input_type: MoveCommandParams::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "MenuQuery",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: MenuContents::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "MiscMoveCommand",
                plugin_name: "RemoteFortressReader",
                input_type: MiscMoveParams::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "MoveCommand",
                plugin_name: "RemoteFortressReader",
                input_type: MoveCommandParams::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "MovementSelectCommand",
                plugin_name: "RemoteFortressReader",
                input_type: IntMessage::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "PassKeyboardEvent",
                plugin_name: "RemoteFortressReader",
                input_type: KeyboardEvent::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "ResetMapHashes",
                plugin_name: "RemoteFortressReader",
                input_type: EmptyMessage::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "SendDigCommand",
                plugin_name: "RemoteFortressReader",
                input_type: DigCommand::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "SetPauseState",
                plugin_name: "RemoteFortressReader",
                input_type: SingleBool::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "SetSideMenu",
                plugin_name: "RemoteFortressReader",
                input_type: SidebarCommand::full_name(),
                output_type: EmptyMessage::full_name(),
            },
        ]
    }
}
///RPC for the "mypluginname" plugin.
pub struct Mypluginname<'a, TChannel: crate::Channel> {
    ///Reference to the client to exchange messages.
    pub channel: &'a mut TChannel,
}
impl<'a, TChannel: crate::Channel> Mypluginname<'a, TChannel> {
    ///Initialize the plugin from a channel to DFHack.
    pub fn new(channel: &'a mut TChannel) -> Self {
        Self { channel }
    }
    ///Method `RenameBuilding` from the plugin `mypluginname`
    pub fn rename_building(
        &mut self,
        request: RenameBuildingIn,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("mypluginname", "RenameBuilding", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `RenameSquad` from the plugin `mypluginname`
    pub fn rename_squad(
        &mut self,
        request: RenameSquadIn,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("mypluginname", "RenameSquad", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
    ///Method `RenameUnit` from the plugin `mypluginname`
    pub fn rename_unit(
        &mut self,
        request: RenameUnitIn,
    ) -> Result<crate::Reply<()>, TChannel::TError> {
        let _response: crate::Reply<EmptyMessage> =
            self.channel
                .request("mypluginname", "RenameUnit", request)?;
        let _response = crate::Reply {
            reply: (),
            fragments: _response.fragments,
        };
        Ok(_response)
    }
}
#[cfg(feature = "reflection")]
impl<TChannel: crate::Channel> crate::reflection::StubReflection for Mypluginname<'_, TChannel> {
    fn list_methods() -> Vec<crate::reflection::RemoteProcedureDescriptor> {
        vec![
            crate::reflection::RemoteProcedureDescriptor {
                name: "RenameBuilding",
                plugin_name: "mypluginname",
                input_type: RenameBuildingIn::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "RenameSquad",
                plugin_name: "mypluginname",
                input_type: RenameSquadIn::full_name(),
                output_type: EmptyMessage::full_name(),
            },
            crate::reflection::RemoteProcedureDescriptor {
                name: "RenameUnit",
                plugin_name: "mypluginname",
                input_type: RenameUnitIn::full_name(),
                output_type: EmptyMessage::full_name(),
            },
        ]
    }
}
