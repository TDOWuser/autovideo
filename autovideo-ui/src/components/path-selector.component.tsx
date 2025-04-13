import { open, OpenDialogOptions } from '@tauri-apps/plugin-dialog';

type Props = {
    value: string[],
    htmlId: string,
    name: string,
    onConfirm: (paths: string[]) => void
    tooltip?: string,
    options?: OpenDialogOptions,
    disabled?: boolean
}
const PathSelector = ({ htmlId, name, options, tooltip, value, onConfirm, disabled }: Props) => {
    const onButtonPress = async () => {
        const file: string | string[] | null = await open(options)
        if (file) {
            if (typeof file === 'string') {
                onConfirm([file])
            } else {
                onConfirm(file)
            }
        }
    }

    return (
        <div style={{display: 'flex', alignItems: 'flex-end', marginBottom: 6, gap: 10}}>
            <div className="field-row-stacked">
                <label htmlFor={htmlId}>{name}</label>
                <input title={tooltip} id={htmlId} type="text" value={value.join(', ')} onChange={e => {if (e.target.value === '') onConfirm([])}} autoComplete='off' disabled={disabled} />
            </div>
            <button disabled={disabled} onClick={onButtonPress}>Select File{options?.multiple ? '(s)' : ''}</button>
        </div>
    )
}

export default PathSelector
