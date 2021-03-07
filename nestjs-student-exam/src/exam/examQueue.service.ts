import { Inject, Injectable } from "@nestjs/common";
import { ClientProviderOptions, ClientProxy, Transport } from "@nestjs/microservices";
import { Observable } from "rxjs";
import { rabbitmqKeys } from "../common/Configuration";
import { AnswerQuestionDto } from "./dto/Answer.dto";
import { FinishExamDto } from "./dto/FinishExam.dto";
import { StartExamDto } from "./dto/StartExam.dto";

const INJECT_TOKEN_EXAM = "INJECT_TOKEN_EXAM";
const QUEUE_PATTERN_EXAM = "q_patterns";
const PATTERN_START_EXAM = "exam_started";
const PATTERN_ANSWER_QUESTION = "question_answered";
const PATTERN_FINISH_EXAM = "exam_finished";

@Injectable()
export class ExamQueueService {
    public constructor(@Inject(INJECT_TOKEN_EXAM) private readonly client: ClientProxy) { }

    public sendStartExam(startExamDto: StartExamDto): Observable<any> {
        console.info("Client send: ", startExamDto)

        return this.client.send(PATTERN_START_EXAM, startExamDto);
    }

    public sendQuestionAnswer(answerQuestionDto: AnswerQuestionDto): Observable<any> {
        console.info("Client send: ", answerQuestionDto)

        return this.client.send(PATTERN_ANSWER_QUESTION, answerQuestionDto);
    }

    public sendfinishExam(finishExamDto: FinishExamDto): Observable<any> {
        console.info("Client send: ", finishExamDto)

        return this.client.send(PATTERN_FINISH_EXAM, finishExamDto);
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