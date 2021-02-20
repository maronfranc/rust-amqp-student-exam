import { MOCK_ID_STUDENT } from "src/common/constants";

interface User {
    idStudent: number;
}

export abstract class BaseController {
    public getUserFromToken(): User {
        return {
            idStudent: MOCK_ID_STUDENT
        }
    }
}