import {default as BaseAxios} from "axios"

export const axios = BaseAxios.create({
  baseURL: import.meta.dev
    ? "http://localhost:8080/api"
    : `/api`,
})