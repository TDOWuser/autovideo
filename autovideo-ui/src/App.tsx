import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "7.css"
import "./App.css"
import { getCurrentWindow } from "@tauri-apps/api/window";
import PathSelector from "./components/path-selector.component";
import { listen } from "@tauri-apps/api/event";
import { message } from "@tauri-apps/plugin-dialog";
import { revealItemInDir } from "@tauri-apps/plugin-opener";

const appWindow = getCurrentWindow()
document.getElementById('minimize-button')?.addEventListener('click', () => appWindow.minimize())
document.getElementById('close-button')?.addEventListener('click', () => appWindow.close())

window.addEventListener('contextmenu', e => e.preventDefault())

function App() {
    const [active, setActive] = useState(false)
    const [progress, setProgress] = useState({current: 0, max: 1, isErrored: false})

    const [modName, setModName] = useState('')
    const [selectedGenerate, setSelectedGenerate] = useState<'esp' | 'script'>('esp')
    const [size, setSize] = useState(512)
    const [fps, setFps] = useState(10)

    const [inputs, setInputs] = useState<string[]>([])
    const [esp, setEsp] = useState<string>()
    const [desp, setDesp] = useState<string>()

    const [espName, setEspName] = useState('')
    const [tvRecord, setTvRecord] = useState('')
    const [prRecord, setPrRecord] = useState('')
    const [driveInEspName, setDriveinEspName] = useState('')

    const [shortNames, setShortNames] = useState(true)
    const [keepAspectRatio, setKeepAspectRatio] = useState(true)

    const inputValid = inputs.length > 0
        && modName.length > 0
        && (selectedGenerate === 'script' ? (espName.length > 0 && tvRecord.length > 0 && prRecord.length > 0) : true)

    const onStart = async () => {
        setActive(true)
        setProgress({current: 0, max: 1, isErrored: false})
        const unlisten = await listen<{ current: number, max: number }>('listener', (event) => setProgress({...event.payload, isErrored: false}))
        try {
            await invoke('convert_files', {
                inputs,
                inputEsp: esp,
                inputEspDriveIn: desp,
                modName,
                inputFramerate: fps,
                shortNames,
                videoName: undefined,
                size,
                keepAspectRatio,
                scriptInfo: selectedGenerate === 'script' ? {
                    esp_name: espName,
                    tv_record: tvRecord,
                    pr_record: prRecord,
                    di_esp_name: driveInEspName
                } : undefined
            })
            await revealItemInDir('./output')
        } catch (err) {
            setProgress({current: 1, max: 1, isErrored: true})
            await message(String(err), { title: 'Error', kind: 'error' })
        }
        unlisten()
        setActive(false)
    }

    return (
        <div className="window-body has-space" style={{height: 280, display: 'flex', flexDirection: 'column', justifyContent: 'space-between'}}>
            <div style={{display: 'flex', justifyContent: 'space-between'}}>
                <div>
                    <div className="field-row-stacked" style={{marginBottom: 6}}>
                        <label htmlFor="mod-name">Mod Name*</label>
                        <input
                            autoComplete="off"
                            id="mod-name"
                            type="text"
                            value={modName}
                            onChange={e => {if (e.target.value.length <= 10) setModName(e.target.value)}}
                            title={`Name of the mod. Can't be longer than 10 characters!`}
                            disabled={active}
                        />
                    </div>
                    <PathSelector
                        value={inputs}
                        onConfirm={setInputs}
                        htmlId="input-path"
                        name="Input Path*"
                        options={{multiple: true, filters: [{name: 'Video', extensions: ['mp4', 'mkv', 'webm', 'avi', 'gif', 'webp']}]}}
                        tooltip={`Path(s) to video(s) to convert.\nNames of video files will be used to name the holotapes.\nVideo names can't be longer than 10 characters!`}
                        disabled={active}
                    />
                    {selectedGenerate === 'esp' && <>
                        <PathSelector
                            value={esp ? [esp] : []}
                            onConfirm={value => setEsp(value.length > 0 ? value[0] : undefined)}
                            htmlId="esp-path"
                            name="ESP Path"
                            options={{filters: [{name: 'CreationKit ESP', extensions: ['esp']}]}}
                            tooltip={`OPTIONAL\nPath to existing esp file to append to that one instead of generating a new one\nThis will create a copy in the output folder and not directly edit given one\nOnly works on ESPs that still have placeholders left`}
                            disabled={active}
                        />
                        <PathSelector
                            value={desp ? [desp] : []}
                            onConfirm={value => setDesp(value.length > 0 ? value[0] : undefined)}
                            htmlId="desp-path"
                            name="DriveIn ESP Path"
                            options={{filters: [{name: 'CreationKit ESP', extensions: ['esp']}]}}
                            tooltip={`OPTIONAL\nPath to existing DriveIn esp file to append to that one instead of generating a new one\nThis will create a copy in the output folder and not directly edit given one\nOnly works on ESPs that still have placeholders left`}
                            disabled={active}
                        />
                    </>}
                    {selectedGenerate === 'script' && <>
                        <div className="field-row-stacked">
                            <label htmlFor="esp-name-input">ESP Name*</label>
                            <input id="esp-name-input" autoComplete="off" type="text" placeholder="your_votw_mod.esp" value={espName} onChange={e => setEspName(e.target.value)} disabled={active} title="Full name of ESP to apply the script to" />
                        </div>
                        <div className="field-row-stacked">
                            <label htmlFor="tv-record-input">TV Record*</label>
                            <input id="tv-record-input" autoComplete="off" type="text" placeholder="03002E88" value={tvRecord} onChange={e => setTvRecord(e.target.value)} title="FormID for any existing TV Activator, doesn't matter which" disabled={active} />
                        </div>
                        <div className="field-row-stacked">
                            <label htmlFor="pr-record-input">Projector Record*</label>
                            <input id="pr-record-input" autoComplete="off" type="text" placeholder="03002E98" value={prRecord} onChange={e => setPrRecord(e.target.value)} title="FormID for any existing Projector Activator, doesn't matter which" disabled={active} />
                        </div>
                    </>}
                </div>
                <div>
                    <fieldset>
                        <legend>ESP / Script</legend>
                        <div className="field-row">
                            <input id="selector-esp" type="radio" name="esp" onChange={() => setSelectedGenerate('esp')} checked={selectedGenerate === 'esp'} disabled={active} />
                            <label htmlFor="selector-esp">Generate esp file</label>
                        </div>
                        <div className="field-row">
                            <input id="selector-script" type="radio" name="script" onChange={() => setSelectedGenerate('script')} checked={selectedGenerate === 'script'} disabled={active} />
                            <label title={`For advanced users. Generates a FO4Edit script to add video records to existing esp. No esps will be generated\nUseful for when you already have an existing VotW esp, either a full one made by autovideo or one you made yourself`} htmlFor="selector-script">Generate script</label>
                        </div>
                    </fieldset>
                    <div style={{display: 'flex', gap: 10}}>
                        <div>
                            <label htmlFor="size-select" style={{marginRight: 5}}>Size</label>
                            <select disabled={active} id="size-select" value={size} onChange={e => setSize(Number(e.target.value))} title={`Size of output frames\nDetermines video resolution in-game. Switch to 256 in case you want to preserve drive space`}>
                                {[128, 256, 512, 1024].map(option => (
                                    <option key={option}>{option}</option>
                                ))}
                            </select>
                        </div>
                        <div>
                            <label htmlFor="fps-select" style={{marginRight: 5}}>FPS</label>
                            <input
                                id="fps-select"
                                type="number"
                                min={1}
                                max={60}
                                value={fps}
                                onChange={e => {
                                    const nr = Number(e.target.value)
                                    if (nr <= 60 && nr >= 1) {
                                        setFps(nr)
                                    }
                                }}
                                title={`Framerate at which to play the videos in-game\nAlternatively you can put the wanted framerate in the video filename like this: video.30.mp4.`}
                                disabled={active}
                            />
                        </div>
                    </div>
                    <fieldset>
                        <legend>Options</legend>
                        <div className="field-row">
                            <input checked={shortNames} onChange={() => setShortNames(b => !b)} type="checkbox" id="short-names" disabled={active} />
                            <label title={`Enable to not give a warning for names being too long and to automatically cut them shorter\nWill not work if multiple videos start with the same 10 characters`} htmlFor="short-names">Short names</label>
                        </div>
                        <div className="field-row">
                            <input checked={keepAspectRatio} onChange={() => setKeepAspectRatio(b => !b)} type="checkbox" id="keep-aspect-ratio" disabled={active} />
                            <label title="Will automatically refit input to 4:3 aspect ratio. (Which fits FO4 TVs better)" htmlFor="keep-aspect-ratio">Keep aspect ratio</label>
                        </div>
                    </fieldset>
                    {selectedGenerate === 'script' && <div className="field-row-stacked">
                        <label htmlFor="di-esp-input">DriveIn ESP Name</label>
                        <input id="di-esp-input" autoComplete="off" type="text" placeholder="your_di_votw_mod.esp" value={driveInEspName} onChange={e => setDriveinEspName(e.target.value)} title="Leave empty if not applicable" disabled={active} />
                    </div>}
                </div>
            </div>
            <div style={{display: 'flex', gap: 10, alignItems: 'center', marginTop: 6}}>
                <div role="progressbar" className={`${active ? (progress.current === 0 ? 'marquee' : 'animate') : ''} ${progress.isErrored ? 'error' : ''}`} style={{width: '100%'}}>
                    <div style={{width: `${progress.current/progress.max*100}%`}}></div>
                </div>
                <button onClick={onStart} disabled={!inputValid || active}>START</button>
            </div>
        </div>
    );
}

export default App;
