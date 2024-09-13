import { useState, useEffect } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';

interface WebSocketHookProps {
    setFetchProgress: (fetchProgress: string) => void;
    shouldConnectToWebsocket: boolean;
}
const useWebSocketHook = ({setFetchProgress, shouldConnectToWebsocket }: WebSocketHookProps) => {
    const [isWebsocketConnected, setIsWebsocketConnected] = useState(false);

    const END_POINT: string = 'ws://localhost:8000/ws';

    const { sendMessage, lastMessage, readyState, getWebSocket } = useWebSocket(END_POINT, {
        shouldReconnect: () => false, // No need, the life time of the WebSocket connection will be handled manually.
        onOpen: () => setIsWebsocketConnected(true),
        onClose: () => setIsWebsocketConnected(false),
        onMessage: (message) => setFetchProgress(message.data),
    }, shouldConnectToWebsocket);

    useEffect(() => {
        setIsWebsocketConnected(readyState === ReadyState.OPEN);
    }, [readyState as ReadyState]);

    useEffect(() => {
        if (lastMessage !== null) {
            // console.log('Received WebSocket message:', lastMessage.data);
            setFetchProgress(lastMessage.data);
        }
    }, [lastMessage]);

    return { isWebsocketConnected  };
};

export default useWebSocketHook;
