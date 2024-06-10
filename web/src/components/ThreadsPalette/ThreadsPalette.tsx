import { PaletteColor } from '../../services/imageService';
import './ThreadsPalette.css';

interface ThreadPaletteProps {
    palette: PaletteColor[];
}

export default function ThreadsPalette({ palette }: ThreadPaletteProps) {
    const calculateThreadLength = (stitches: number) =>
        stitches + stitches * 0.2;

    return (
        <div className='palette-container'>
            <ul>
                {palette.map((thread, index) => (
                    <li className={'thread-container'} key={index}>
                        <p style={{ textAlign: 'center' }}>
                            Color {thread.identifier}:{' '}
                            <a
                                rel='noopener'
                                href={`https://stitchpalettes.com/embroidery-thread-color-schemes/?threadcode=${thread.color.name}`}
                            >
                                #{thread.color.name}
                            </a>
                        </p>
                        <div
                            className='thread-color-sample'
                            style={{
                                backgroundColor: `rgba(${thread.color.rgb.toString()},1)`,
                            }}
                        />
                        <p>{calculateThreadLength(thread.nStitches)}cm</p>
                    </li>
                ))}
            </ul>
        </div>
    );
}
