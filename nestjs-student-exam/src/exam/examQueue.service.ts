import { Inject, Injectable } from "@nestjs/common";
import { ClientProviderOptions, ClientProxy, Transport } from "@nestjs/microservices";
import { Observable } from "rxjs";
import { rabbitmqKeys } from "../common/Configuration";
import { ExamDto } from "./dto/Exam.dto";

const INJECT_TOKEN_EXAM = "INJECT_TOKEN_EXAM";
const QUEUE_PATTERN_EXAM = "q_pattern_exam";
const PATTERN_START_EXAM = "start_exam";
const PATTERN_ANSWER_QUESTION = "answer_question";
const PATTERN_FINISH_EXAM = "finish_exam";

@Injectable()
export class ExamQueueService {
    public constructor(@Inject(INJECT_TOKEN_EXAM) private readonly client: ClientProxy) { }

    public sendStartQueue(examData: ExamDto): Observable<any> {
        return this.client.send(PATTERN_START_EXAM, examData);
    }

    public sendQuestionAnswer(questionAnswer: ExamDto): Observable<any> {
        return this.client.send(PATTERN_ANSWER_QUESTION, questionAnswer);
    }

    public finishExam(examData: ExamDto): Observable<any> {
        return this.client.send(PATTERN_FINISH_EXAM, examData);
    }
}

const { host, password, port, username, vhost } = rabbitmqKeys;

const commonProducer: Readonly<ClientProviderOptions> = {
    transport: Transport.RMQ,
    name: INJECT_TOKEN_EXAM,
    options: {
        urls: [`amqp://${username}:${password}@${host}:${port}/${vhost}`],
    },
};

export const startExamProducer: ClientProviderOptions = {
    ...commonProducer,
    options: {
        ...commonProducer.options,
        queue: QUEUE_PATTERN_EXAM,
        queueOptions: {
            durable: true
        }
    }
};