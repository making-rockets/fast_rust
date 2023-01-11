

const api = axios.create({
    baseURL: 'http://127.0.0.1:3000', // 请求的公共地址部分
    timeout: 3000 // 请求超时时间 当请求时间超过5秒还未取得结果时 提示用户请求超时
})

// interceptors axios的拦截器对象
api.interceptors.request.use(config => {
    // config 请求的信息
    return config // 将配置完成的config对象返回出去 如果不返回 请求则不会进行
}, err => {
    // 请求发生错误时的处理 抛出错误
    Promise.reject(err)
})
api.interceptors.response.use(res => {
    // 我们一般在这里处理，请求成功后的错误状态码 例如状态码是500，404，403
    // res 是所有相应的信息
    console.log(res)
    return Promise.resolve(res)
}, err => {
    // 服务器响应发生错误时的处理
    Promise.reject(err)
})


class Student {
    constructor(studentId, name, clzz, DOB, parentName, mobile, address) {
        this.studentId = studentId;
        this.name = name;
        this.clzz = clzz;
        this.DOB = DOB;
        this.parentName = parentName;
        this.mobile = mobile;
        this.address = address;
    }

    getStudent(studentId) {
        //    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 sadfdsf
        // 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

        let student = api.get(`/admin/student/edit-student`, {studentId}).then(res => {
            return res;
        })
        return student;
    }

}
