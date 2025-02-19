use std::fs;
use crate::utility::user_input;

pub fn generate_script(mod_name: &str, mod_id: &str, video_data: &[(String, String, String, bool)]) -> Result<(), String> {
    println!("\nPlease provide the following information to generate the script:");
    let esp_name = user_input("Mod file name (with extension): ");
    let esp_name = esp_name.trim();
    let tv_record = user_input("FormID for any TV Activator: ");
    let tv_record = tv_record.trim();
    let pr_record = user_input("FormID for any Projector Activator: ");
    let pr_record = pr_record.trim();
    let di_esp_name = if video_data.iter().any(|(_, _, _, has_drivein)| *has_drivein) {
        user_input("DriveIn mod file name (with extension, leave empty if none): ").trim().to_string()
    } else {
        "".to_string()
    };
    
    let di_enabled = !di_esp_name.is_empty();
    
    let mut video_handlers = String::new();
    let mut di_handlers = String::new();
    for (video_id, video_name, audio_name, has_drivein) in video_data {
        video_handlers.push_str(&format!("  HandleVideo('{video_id}', '{video_name}', '{audio_name}');\n"));
        if di_enabled && *has_drivein {
            di_handlers.push_str(&format!("  HandleDIVideo('{video_id}', '{video_name}', '{audio_name}');\n"))
        }
    }
    let script_contents = format!("unit UserScript;

var
  TargetPlugin: IInterface;
  NoteFormList, TVFormList, PRFormList: IInterface;
  MiscFormList, ScreenFormList, SoundFormList: IInterface;
  modId, modName: string;

function Initialize: Integer;
var
  i: Integer;
  FLSTGroup: IInterface;
begin
  begin
    for i := 0 to FileCount - 1 do begin
      if SameText(GetFileName(FileByIndex(i)), '{esp_name}') then begin
        TargetPlugin := FileByIndex(i);
        Break;
      end;
    end;
    if not Assigned(TargetPlugin) then begin
      AddMessage('Plugin not found');
      Result := 1;
      Exit;
    end;
  end;
  
  FLSTGroup := GroupBySignature(TargetPlugin, 'FLST');
  
  NoteFormList := ElementByIndex(FLSTGroup, 1);
  if not Assigned(NoteFormList) then begin
    AddMessage('FormID List not found');
    Result := 1;
    Exit;
  end;
  TVFormList := ElementByIndex(FLSTGroup, 2);
  if not Assigned(TVFormList) then begin
    AddMessage('FormID List not found');
    Result := 1;
    Exit;
  end;
  PRFormList := ElementByIndex(FLSTGroup, 0);
  if not Assigned(PRFormList) then begin
    AddMessage('FormID List not found');
    Result := 1;
    Exit;
  end;
  
  modId := '{mod_id}';
  modName := '{mod_name}';
  
{video_handlers}

  if {} then begin
    ProcessDI();
  end;

  Result := 0; // Return 0 to indicate success
end;



function ProcessDI: Integer;
var
  i: Integer;
  FLSTGroup: IInterface;
begin
  TargetPlugin := nil;
  begin
    for i := 0 to FileCount - 1 do begin
      if SameText(GetFileName(FileByIndex(i)), '{di_esp_name}') then begin
        TargetPlugin := FileByIndex(i);
        Break;
      end;
    end;
    if not Assigned(TargetPlugin) then begin
      AddMessage('Plugin not found');
      Result := 1;
      Exit;
    end;
  end;
  
  FLSTGroup := GroupBySignature(TargetPlugin, 'FLST');
  
  MiscFormList := ElementByIndex(FLSTGroup, 0);
  if not Assigned(MiscFormList) then begin
    AddMessage('FormID List not found');
    Result := 1;
    Exit;
  end;
  ScreenFormList := ElementByIndex(FLSTGroup, 1);
  if not Assigned(ScreenFormList) then begin
    AddMessage('FormID List not found');
    Result := 1;
    Exit;
  end;
  SoundFormList := ElementByIndex(FLSTGroup, 2);
  if not Assigned(SoundFormList) then begin
    AddMessage('FormID List not found');
    Result := 1;
    Exit;
  end;

{di_handlers}
  
  Result := 0;
end;



procedure HandleVideo(videoId, videoName, audioName: string);
var
  NewNote, NewTVACTI, NewPRACTI, NewSound, NewCOBJ: IInterface;
begin
  // Handle NOTE
  NewNote := CopyLastRecord(TargetPlugin, 'NOTE');
  if not Assigned(NewNote) then begin
    AddMessage('NOTE record not found');
    Exit;
  end;
  SetElementEditValues(NewNote, 'EDID', 'VotW_'+modName+videoName+'_Tape');
  SetElementEditValues(NewNote, 'FULL', '[VotWH] '+modName+' - '+videoName);
  AddMessage('NOTE record added successfully.');
  AddToFormIDList(NoteFormList, NewNote);
  
  // Handle ACTI TV
  NewTVACTI := CopyRecordByFormID(TargetPlugin, '{tv_record}');
  if not Assigned(NewTVACTI) then begin
    AddMessage('TV ACTI record not found');
    Exit;
  end;
  SetElementEditValues(NewTVACTI, 'EDID', 'VotW_TV'+modName+videoName+'Screen');
  SetElementEditValues(NewTVACTI, 'Model\\MODL', 'Videos\\Television\\'+modId+'\\'+videoId+'.nif');
  AddToFormIDList(TVFormList, NewTVACTI);
  
  // Handle ACTI PR
  NewPRACTI := CopyRecordByFormID(TargetPlugin, '{pr_record}');
  if not Assigned(NewPRACTI) then begin
    AddMessage('PR ACTI record not found');
    Exit;
  end;
  SetElementEditValues(NewPRACTI, 'EDID', 'VotW_PR'+modName+videoName+'Screen');
  SetElementEditValues(NewPRACTI, 'Model\\MODL', 'Videos\\Projector\\'+modId+'\\'+videoId+'.nif');
  AddToFormIDList(PRFormList, NewPRACTI);
  
  // Handle SNDR
  NewSound := CopyLastRecord(TargetPlugin, 'SNDR');
  if not Assigned(NewSound) then begin
    AddMessage('SNDR record not found');
    Exit;
  end;
  SetElementEditValues(NewSound, 'EDID', 'VotW_'+modId+videoId+'Sound');
  SetEditValue(ElementByPath(ElementByIndex(ElementByPath(NewSound, 'Sounds'), 0), 'ANAM'), 'data\\sound\\Videos\\'+modId+'\\'+audioName);
  
  // Handle COBJ
  NewCOBJ := CopyLastRecord(TargetPlugin, 'COBJ');
  if not Assigned(NewCOBJ) then begin
    AddMessage('COBJ record not found');
    Exit;
  end;
  SetElementEditValues(NewCOBJ, 'EDID', 'VotW_'+modName+videoName+'Craft');
  SetElementEditValues(NewCOBJ, 'CNAM', GetEditValue(NewNote));
end;



procedure HandleDIVideo(videoId, videoName, audioName: string);
var
  NewScreen, NewMisc, NewCOBJ, NewSound: IInterface;
begin
  // Handle Screen
  NewScreen := CopyLastRecord(TargetPlugin, 'ACTI');
  if not Assigned(NewScreen) then begin
    AddMessage('TV ACTI record not found');
    Exit;
  end;
  SetElementEditValues(NewScreen, 'EDID', 'VotW_DI'+modName+videoName+'Screen');
  SetElementEditValues(NewScreen, 'Model\\MODL', 'Videos\\DriveIn\\'+modId+'\\'+videoId+'.nif');
  AddToFormIDList(ScreenFormList, NewScreen);
  
  // Handle MISC
  NewMisc := CopyLastRecord(TargetPlugin, 'MISC');
  if not Assigned(NewMisc) then begin
    AddMessage('MISC record not found');
    Exit;
  end;
  SetElementEditValues(NewMisc, 'EDID', 'VotW_DI'+modName+videoName+'Reel');
  SetElementEditValues(NewMisc, 'FULL', '[VotWR] '+modName+' - '+videoName);
  AddMessage('MISC record added successfully.');
  AddToFormIDList(MiscFormList, NewMisc);
  
  // Handle COBJ
  NewCOBJ := CopyLastRecord(TargetPlugin, 'COBJ');
  if not Assigned(NewCOBJ) then begin
    AddMessage('COBJ record not found');
    Exit;
  end;
  SetElementEditValues(NewCOBJ, 'EDID', 'VotW_DI'+modName+videoName+'Craft');
  SetElementEditValues(NewCOBJ, 'CNAM', GetEditValue(NewMisc));
  
  // Handle SNDR
  NewSound := CopyLastRecord(TargetPlugin, 'SNDR');
  if not Assigned(NewSound) then begin
    AddMessage('SNDR record not found');
    Exit;
  end;
  SetElementEditValues(NewSound, 'EDID', 'VotW_DI'+modName+videoName+'Sound');
  SetEditValue(ElementByPath(ElementByIndex(ElementByPath(NewSound, 'Sounds'), 0), 'ANAM'), 'data\\sound\\Videos\\'+modId+'\\'+audioName);
  AddToFormIDList(SoundFormList, NewSound);
end;



// Procedure to append the NOTE to the FormID List
procedure AddToFormIDList(aFormIdList, aRecord: IInterface);
var
  Entries, NewEntry: IInterface;
  GlobalFormID: string;
begin
  // Get the Entries subrecord of the FormID List
  Entries := ElementByPath(aFormIdList, 'FormIDs');
  
  if not Assigned(Entries) then begin
    AddMessage('FormID List entries not found!');
    Exit;
  end;

  // Get the correct load order FormID of the new record
  GlobalFormID := IntToHex(GetLoadOrderFormID(aRecord), 8);
  
  // Add a new FormID entry to the list
  NewEntry := ElementAssign(Entries, HighInteger, nil, False);  // Appends a new entry
  SetEditValue(NewEntry, GlobalFormID);
  
  AddMessage('Added ' + GlobalFormID + ' to FormID list.');
end;



function CopyLastRecord(aFile: IInterface; recordType: string): IInterface;
var
  i: Integer;
  Group, sourceRecord, copiedRecord: IInterface;
begin
  Result := nil;
  // Get the group from the plugin
  Group := GroupBySignature(aFile, recordType);
  
  sourceRecord := ElementByIndex(Group, ElementCount(Group) - 1);
  copiedRecord := wbCopyElementToFile(sourceRecord, aFile, True, True);
  if Assigned(copiedRecord) then begin
    AddMessage('Copied record');
    Result := copiedRecord;
  end;

  if not Assigned(Result) then
    AddMessage('record not found');
end;



function CopyRecordByFormID(aFile: IInterface; FormID: string): IInterface;
var
  i: Integer;
  sourceRecord, copiedRecord: IInterface;
begin
  Result := nil;
  
  sourceRecord := RecordByFormID(TargetPlugin, HexStrToInt(FormID), True);
  copiedRecord := wbCopyElementToFile(sourceRecord, aFile, True, True);
  if Assigned(copiedRecord) then begin
    AddMessage('Copied record');
    Result := copiedRecord;
  end;

  if not Assigned(Result) then
    AddMessage('record with FormID ' + FormID + ' not found.');
end;



function HexStrToInt(HexStr: string): Integer;
begin
  try
    Result := StrToInt64('$' + HexStr);  // Prepend with $ to signify hex
  except
    on E: Exception do begin
      AddMessage('Error converting FormID: ' + HexStr + ' to integer. ' + E.Message);
      Result := 0; // Return 0 on error
    end;
  end;
end;
end.", if di_enabled { "True" } else { "False" });

    match fs::write("output/script.txt", script_contents) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to write script file!".to_string()),
    }
}