import { useState, useEffect } from 'react';
import useWebSocket from 'react-use-websocket';

const useWebSocketHook = () => {
    const [progress, setProgress] = useState<string>('');

    const { sendMessage, lastMessage, readyState } = useWebSocket('http://localhost:8080/ws');

    useEffect(() => {
        if (lastMessage !== null) {
            setProgress(lastMessage.data);
        }
    }, [lastMessage]);

    return { progress };
};

export default useWebSocketHook;