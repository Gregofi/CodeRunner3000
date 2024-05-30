import React from 'react';

export function CheckBox({
    onChange,
    label,
}: {
    onChange: (checked: boolean) => void;
    label: string;
}) {
    const [checked, setChecked] = React.useState(false);
    return (
        <div>
            <input
                type="checkbox"
                checked={checked}
                onChange={() => {
                    onChange(!checked);
                    setChecked(!checked);
                }}
            />
            <label>{label}</label>
        </div>
    );
}
