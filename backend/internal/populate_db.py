import requests
from dataclasses import dataclass
from typing import List, Tuple, Dict, Any

BIND_ADDRESS: str = "http://localhost:8000/fetch-chess-data" 

@dataclass
class BackendRequestPayload:
    user_name: str
    games_count: int
    game_mode: str
    user_color: str
    
    def to_json(self) -> Dict[str, Any]:
        return {
            "username": self.user_name,
            "games_count": self.games_count,
            "game_mode": self.game_mode,
            "user_color": self.user_color,
        }
    
    @staticmethod
    def from_json(data: Dict[str, Any]) -> "BackendRequestPayload":
        return BackendRequestPayload(
            user_name = data["username"],
            games_count = data["games_count"],
            game_mode = data["game_mode"],
            user_color = data["user_color"]
        )

@dataclass
class BackendResponse:
    average_time: float
    opponents_and_their_rating: List[Tuple[str, int]]
    
    @staticmethod
    def from_json(data: Dict[str, Any]) -> "BackendResponse":
        return BackendResponse(
            average_time = data["time"],
            opponents_and_their_rating = [
                (opponent[0], opponent[1]) for opponent in data["players_considered"]
            ]
        )

def process_user(
    user_name: str,
    games_count: int,
    game_mode: str, 
    user_color: str) -> BackendResponse:
    payload = BackendRequestPayload(
        user_name = user_name,
        games_count = games_count,
        game_mode = game_mode,
        user_color = user_color
    )
    try:
        response = requests.post(
            BIND_ADDRESS, 
            headers = {
                "Content-Type": 'application/json',
                "x-requested-by": "internal"
            },
            json = payload.to_json())
        response.raise_for_status() 
        response_data = response.json()
        
        return BackendResponse.from_json(response_data)
    except Exception as e:
        print(f"Failed to process user {user_name}: {e}")
        raise

def get_starting_usernames() -> List[str]:
    return ["hexaquarks1", "fifthart"]

if __name__ == "__main__":
    usernames = get_starting_usernames()
    results = []
    
    while len(usernames) != 0:
        user = usernames.pop()
        
        try: 
            result: BackendResponse = process_user(user, 10, "blitz", "both")
            results.append(result)
        except Exception as e:
            print(f"An error occured while trying to process player {user}: {e}")
    
    print("Script finished running. Results:")
    print(results)
    
        