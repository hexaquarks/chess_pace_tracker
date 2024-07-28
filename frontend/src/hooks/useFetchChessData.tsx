import { useState, useEffect } from 'react';
import { sendDataToBackend, ResponseInformation } from '../services/apiService';
import useWebSocketHook from './useWebSocketHook';
import { InputProps } from '../components/panels/InputsPanel';

const useFetchChessData = () => {
    const [response, setResponse] = useState<ResponseInformation | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [usernameNotFound, setUsernameNotFound] = useState(false);
    const [shouldConnectToWebsocket, setShouldConnectToWebsocket] = useState(false);
    const [fetchArgs, setFetchArgs] = useState<InputProps | null>(null);
    const [fetchProgress, setFetchProgress] = useState('');
    const { isWebsocketConnected, closeWebsocketConnection } = useWebSocketHook({setFetchProgress, shouldConnectToWebsocket});

    const fetchData = async (props: InputProps) => {
        // When this POST request is invoked, the WebSocket connection should already be established.
        if (!isWebsocketConnected) {
            // websocket connection not established.
            setError('Connection not established, please check your internet connection and try again.'); 
            return;
        }

        setIsLoading(true);
        setError(null);

        try {
            const responseData: Readonly<ResponseInformation> = await sendDataToBackend(
                props.username,
                props.gamesCount,
                props.gameMode,
                props.userColor,
                setUsernameNotFound
            );
            setResponse(responseData);
        } catch (error) {
            setError(error instanceof Error ? error.message : 'An unexpected error occurred');
        } finally {
            setIsLoading(false);

            // The POST request is completed, we close the Websocket connection to prevent overload.
            closeWebsocketConnection(); 
        }
    };

    useEffect(() => {
        if (shouldConnectToWebsocket && isWebsocketConnected && fetchArgs) {
            // Start the POST request once WebSocket connection is established and all props are set.
            fetchData(fetchArgs);
        }
    }, [shouldConnectToWebsocket, isWebsocketConnected, fetchArgs]);

    const initiateFetch = (props: InputProps) => {
        // Store props for use after the websocket connection is set.
        setFetchArgs(props); 

        setShouldConnectToWebsocket(true); 
        setFetchProgress(''); // Reset progress in between runs.
    };

    return { response, isLoading, error, fetchData: initiateFetch, usernameNotFound, fetchProgress };
};

export default useFetchChessData;