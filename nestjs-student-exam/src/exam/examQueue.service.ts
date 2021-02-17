import { Inject, Injectable } from "@nestjs/common";
import { ClientProviderOptions, ClientProxy, Transport } from "@nestjs/microservices";
import { Observable } from "rxjs";
import { rabbitmqKeys } from "../common/Configuration";
import { ExamDto } from "./dto/Exam.dto";

export const EXAM_INJECT_TOKEN = "EXAM_INJECT_TOKEN";
export const START_EXAM_QUEUE = "q_start_exam";
export const patternStartExam = "start_exam";

@Injectable()
export class ExamQueueService {
    public constructor(@Inject(EXAM_INJECT_TOKEN) private readonly client: ClientProxy) { }

    public sendStartQueue(examData: ExamDto): Observable<any> {
        examData.routing_key = `r_exam_${examData.id}`;

        return this.client.send(patternStartExam, examData);
    }
}

const { host, password, port, username, vhost } = rabbitmqKeys;

export const examProducer: ClientProviderOptions = {
    transport: Transport.RMQ,
    name: EXAM_INJECT_TOKEN,
    options: {
        urls: [`amqp://${username}:${password}@${host}:${port}/${vhost}`],
        queue: START_EXAM_QUEUE,
        queueOptions: {
            durable: true
        }
    },
};
