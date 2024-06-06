import './MenuButton.css';

interface FormButtonInterface {
    type: 'submit' | 'button';
    backgroundColor: string;
    onClick?: () => void;
    children: string;
}

export default function MenuButton({
    type,
    children,
    backgroundColor,
    onClick,
}: FormButtonInterface) {
    return (
        <button
            className='menu-button'
            style={{ backgroundColor }}
            type={type}
            onClick={onClick}
        >
            {children}
        </button>
    );
}
